use actix_web::{web, HttpResponse};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use uuid::Uuid;

use crate::store::Store;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewAccount {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
}

impl Account {
    fn new(name: &str) -> Self {
        Account {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            name: name.to_string(),
        }
    }
}

pub async fn register(user: web::Json<NewAccount>, store: web::Data<Store>) -> HttpResponse {
    let store = store.get_ref().to_owned();
    let account = Account::new(&user.name);
    match Store::insert_user(store, account).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
