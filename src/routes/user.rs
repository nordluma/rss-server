use actix_web::{web, HttpResponse, Responder};

pub fn users(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/users").route(web::get().to(get_user)));
}

async fn get_user() -> impl Responder {
    HttpResponse::Ok().body("getting user".to_owned())
}
