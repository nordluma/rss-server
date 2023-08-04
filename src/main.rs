use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Result};
use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};
use sqlx::PgPool;

mod routes;

const ADDR: &str = "127.0.0.1";

#[derive(Deserialize)]
struct DatabaseSettings {
    username: String,
    password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    host: String,
    database_name: String,
    require_ssl: bool,
}

impl DatabaseSettings {
    fn new() -> DatabaseSettings {
        // use dummy data now, will switch to a config file later
        DatabaseSettings {
            username: "test".to_string(),
            password: "test".to_string().into(),
            port: 5432,
            host: "postgres".to_string(),
            database_name: "test".to_string(),
            require_ssl: true,
        }
    }
    fn connect_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let db_configs = DatabaseSettings::new();
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(db_configs.connect_db());

    let listener = TcpListener::bind(ADDR)?;

    run(listener, connection_pool)?.await?;

    Ok(())
}

fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(db_pool);

    let server = HttpServer::new(move || {
        App::new()
            .configure(routes::user::users)
            .route("/healthcheck", web::get().to(health_check))
            .route(
                "/register",
                web::post().to(routes::authentication::register),
            )
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}
