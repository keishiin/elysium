use actix_web::web;
use actix_session::Session;
use uuid::Uuid;
use serde_json::json;
use actix_web::{web::Json, get, post, HttpResponse};

use crate::models::users::{User, UserRequest};
use crate::utils::errors::ApiError;

#[get("/user")]
async fn get_user(user_id: Json<UserRequest>) -> HttpResponse {
    let user = User::get_user_by_id(&user_id.username);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_err) => HttpResponse::InternalServerError().body("User not found")
    }
}

#[post("/signup")]
async fn signup(new_user: Json<User>) -> HttpResponse {
    let user = User::create_user(new_user.into_inner());

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[post("/signin")]
async fn signin(user_info: Json<UserRequest>, session: Session) -> Result<HttpResponse, ApiError> {
    let user = User::get_user_by_username(&user_info.username)
        .map_err(|e| {
            match e.status_code {
                404 => ApiError::new(401, "Credentials not valid!".to_string()),
                _ => e,
            }
        })?;

    let is_correct_user = user.verify_password(user_info.password.as_bytes())?;
    
    if is_correct_user {
        session.insert("user_id", &user.id).expect("session insert error");
        session.renew();
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/signout")]
async fn singout(session: Session) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = session.get("user_id").expect("session get error");

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({"message": "Signout success!"})))
    } else { 
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}


pub fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
        .service(get_user)
        .service(signup)
        .service(signin)
    );
}