use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, get, post, HttpResponse};

use crate::{models::users, repository::database::Database};

#[get("/user")]
async fn get_user(user_id: Json<users::UserRequest>, data: Data<Database>) -> HttpResponse {
    let user = data.get_user_by_id(&user_id.id);

    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::InternalServerError().body("User not found")
        
    }
}

#[post("/user")]
async fn create_user(new_user: Json<users::User>, database: Data<Database>) -> HttpResponse {
    let user = database.create_user(new_user.into_inner());

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