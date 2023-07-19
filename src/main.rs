use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(users)
            .route("/healthcheck", web::get().to(health_check))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/users")
            .route(web::post().to(create_user))
            .route(web::get().to(get_user)),
    );
}

#[derive(Deserialize)]
struct User {
    name: String,
}

async fn create_user(user: web::Json<User>) -> Result<String> {
    Ok(format!("Creating user: {}", user.name))
}

async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("getting user".to_owned())
}
