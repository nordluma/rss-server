use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpMessage};

pub struct Authenticated(ApiKey);

#[derive(Clone)]
pub struct ApiKey {
    pub key: String,
}

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let user = req.extensions().get::<ApiKey>().cloned();
        let res = match user {
            Some(user) => Ok(Authenticated(user)),
            None => Err(ErrorUnauthorized("Invalid token")),
        };

        ready(res)
    }
}

impl std::ops::Deref for Authenticated {
    type Target = ApiKey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
