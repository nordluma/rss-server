use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Row,
};

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

    pub async fn get_users(self) -> Result<Vec<Account>, sqlx::Error> {
        match sqlx::query("SELECT * FROM users")
            .map(|row: PgRow| Account {
                id: row.get("id"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
                name: row.get("name"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(accounts) => Ok(accounts),
            Err(e) => Err(e),
        }
    }
}
