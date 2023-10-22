use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::Store;

#[derive(Debug, Deserialize)]
pub struct NewFeed {
    pub name: String,
    pub url: String,
    pub user_id: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Feed {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
    pub url: String,
    pub user_id: Uuid,
    pub last_fetched_at: Option<DateTime<Utc>>,
}

impl From<NewFeed> for Feed {
    fn from(value: NewFeed) -> Self {
        Self {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: value.name,
            url: value.url,
            user_id: value.user_id,
            last_fetched_at: None,
        }
    }
}

pub fn feed(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/feed")
            .route(web::get().to(get_feeds))
            .route(web::post().to(create_feed)),
    );
}

pub async fn get_feeds(store: web::Data<Store>) -> HttpResponse {
    let store = store.get_ref().to_owned();
    match store.get_feeds().await {
        Ok(feeds) => HttpResponse::Ok().json(feeds),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_feed(store: web::Data<Store>, _feed: web::Form<NewFeed>) -> HttpResponse {
    let store = store.get_ref().to_owned();
    match store.insert_feed().await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
