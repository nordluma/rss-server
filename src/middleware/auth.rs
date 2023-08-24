use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::{ErrorInternalServerError, ErrorUnauthorized},
    http::header::AUTHORIZATION,
    web, Error as ActixError, HttpMessage,
};
use futures_util::{future::LocalBoxFuture, FutureExt};

use crate::store::Store;

pub struct AuthMiddlewareFactory;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = ActixError> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = ActixError;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // This is still messy, will fix it later when I find a better way
        let srv = self.service.clone();
        let auth_header_opt = req.headers().get(AUTHORIZATION).cloned();

        async move {
            let auth_token = match auth_header_opt {
                Some(token) => token.to_str().unwrap_or("").to_string(),
                None => return Err(ErrorUnauthorized("Invalid token")),
            };

            let api_key = match get_api_key(auth_token.as_str()) {
                Ok(key) => key,
                Err(_) => return Err(ErrorUnauthorized("Invalid token")),
            };

            let store = match req.app_data::<web::Data<Store>>() {
                Some(store) => store.get_ref().clone(),
                None => unreachable!(),
            };

            let opt_user = match Store::get_user_by_api_key(store, api_key).await {
                Ok(user) => user,
                Err(e) => return Err(ErrorInternalServerError(e)),
            };

            if opt_user.is_none() {
                return Err(ErrorUnauthorized("Invalid token"));
            }

            req.extensions_mut().insert(opt_user.unwrap());

            srv.call(req).await
        }
        .boxed_local()
    }
}

fn get_api_key(header: &str) -> Result<&str, ActixError> {
    if header.is_empty() {
        return Err(ErrorUnauthorized("Invalid Token"));
    }

    let tokens_parts = header.split_whitespace().collect::<Vec<_>>();

    if tokens_parts.len() < 2 {
        return Err(ErrorUnauthorized("Invalid Token"));
    }

    if tokens_parts[0] != "ApiKey" {
        return Err(ErrorUnauthorized("Invalid Token"));
    }

    Ok(tokens_parts[1])
}
