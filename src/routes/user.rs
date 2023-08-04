use actix_web::{web, HttpResponse};

use crate::store::Store;

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(get_users)));
}

async fn get_users(store: web::Data<Store>) -> HttpResponse {
    let store = store.get_ref().to_owned();
    match store.get_users().await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
