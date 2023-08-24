use std::future::{ready, Ready};

use actix_web::{error::ErrorUnauthorized, Error, FromRequest, HttpMessage};

use crate::routes::authentication::Account;

pub struct Authenticated(Account);

impl FromRequest for Authenticated {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let user = req.extensions().get::<Account>().cloned();
        let res = match user {
            Some(user) => Ok(Authenticated(user)),
            None => Err(ErrorUnauthorized("Invalid token")),
        };

        ready(res)
    }
}

impl std::ops::Deref for Authenticated {
    type Target = Account;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
