use std::env;
use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Result};
use store::Store;

mod routes;
mod store;

const ADDR: &str = "127.0.0.1";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let store = Store::new(env::var("DATABASE_URL").unwrap().as_str()).await;
    dotenv::dotenv().ok();

    let listener = TcpListener::bind(ADDR)?;

    run(listener, store.unwrap())?.await?;

    Ok(())
}

fn run(listener: TcpListener, store: Store) -> Result<Server, std::io::Error> {
    let db_pool = web::Data::new(store);

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
