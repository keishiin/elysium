use actix_web::web;
use actix_web::{ get, HttpResponse, Responder, Result};

use crate::models::response::Response;

#[get("/")]
async fn index() -> impl Responder {
    let response = Response {
        message: "use get and post on api/user".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(index)
            .service(healthcheck)
    );
}