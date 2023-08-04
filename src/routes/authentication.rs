use actix_web::{web, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NewUser {
    name: String,
}

pub async fn register(user: web::Json<NewUser>) -> Result<String> {
    /*
     * sqlx::query!(
     *      r#"
     *          INSERT INTO user (id, created_at, updated_at, name)
     *          VALUES ($1, $2, $3, $4)
     *      "
     * )
     */

    Ok(format!("Creating user: {}", user.name))
}
