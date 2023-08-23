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
pub struct CreateAccount {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub name: String,
    pub api_key: String,
}

impl CreateAccount {
    fn new(name: &str) -> Self {
        CreateAccount {
            id: Uuid::new_v4(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
            name: name.to_string(),
        }
    }
}

pub async fn register(user: web::Json<NewAccount>, store: web::Data<Store>) -> HttpResponse {
    let store = store.get_ref().to_owned();
    let account = CreateAccount::new(&user.name);

    match Store::insert_user(store, account).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}

#[allow(unused)]
pub async fn get_user_by_api_key(api_key: &str, store: web::Data<Store>) -> HttpResponse {
    let store = store.get_ref().to_owned();

    let user = match Store::get_user_by_api_key(store, api_key).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::Unauthorized().finish(),
    }
}
