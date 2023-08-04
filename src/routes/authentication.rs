use actix_web::{web, HttpResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::store::Store;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewAccount {
    name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub name: String,
}

impl Account {
    fn new(name: &str) -> Self {
        Account {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            name: name.to_string(),
        }
    }
}

pub async fn register(user: web::Json<NewAccount>, store: web::Data<Store>) -> HttpResponse {
    let account = Account::new(&user.name);
    match Store::insert_user(store.get_ref().clone(), account).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::BadRequest().finish(),
    }
}
