use actix_web::{get, web, HttpResponse, Responder, Result};
use serde_json::json;

use crate::models::response::Response;

#[get("/")]
async fn index() -> impl Responder {
    return HttpResponse::Ok().json(json!({"message": "use get and post on api/user"}));
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    return HttpResponse::Ok().json(response);
}

pub async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    return Ok(HttpResponse::NotFound().json(response));
}

pub fn init_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api").service(index).service(healthcheck));
}
