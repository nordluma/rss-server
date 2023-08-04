use actix_web::{web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::Store;

#[derive(Debug, Deserialize, Serialize)]
pub struct Feed {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub url: String,
    pub user_id: Uuid,
    pub last_fetched_at: Option<NaiveDateTime>,
}

pub fn feed(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/feed").route(web::get().to(get_feeds)));
}

pub async fn get_feeds(store: web::Data<Store>) -> HttpResponse {
    let store = store.get_ref().to_owned();
    match store.get_feeds().await {
        Ok(feeds) => HttpResponse::Ok().json(feeds),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
