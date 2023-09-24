use actix_web::web;
use actix_web::{web::Json, get, post, HttpResponse, Responder, Result};

use crate::models::users::{User, UserRequest};
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

#[get("/user")]
async fn get_user(user_id: Json<UserRequest>) -> HttpResponse {
    let user = User::get_user_by_id(&user_id.id);

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::InternalServerError().body("User not found")
        
    }
}

#[post("/user")]
async fn create_user(new_user: Json<User>) -> HttpResponse {
    let user = User::create_user(new_user.into_inner());

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        
    }
}

pub fn config( cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_user)
            .service(get_user)
    );
}