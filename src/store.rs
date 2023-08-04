use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::routes::authentication::Account;

#[derive(Debug, Clone)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Result<Self, sqlx::Error> {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection: {}", e),
        };

        Ok(Store {
            connection: db_pool,
        })
    }

    pub async fn insert_user(self, account: Account) -> Result<bool, sqlx::Error> {
        match sqlx::query(
            "INSERT INTO users (id, created_at, updated_at, name, api_key)
            VALUES ($1, $2, $3, $4, encode(random()::text:bytea), 'hex')
            RETURNING *",
        )
        .bind(account.id)
        .bind(account.created_at)
        .bind(account.updated_at)
        .bind(account.name)
        .fetch_one(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(e),
        }
    }
}
