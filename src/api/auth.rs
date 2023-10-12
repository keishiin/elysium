use actix_session::Session;
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::users::{User, UserRequest};
use crate::utils::errors::ApiError;

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
    let user =
        User::get_user_by_username(&user_info.username).map_err(|e| match e.status_code {
            404 => ApiError::new(401, "Credentials not valid!".to_string()),
            _ => e,
        })?;

    let is_correct_user = user.verify_password(user_info.password.as_bytes())?;

    if is_correct_user {
        session
            .insert("user_id", &user.id)
            .expect("session insert error");
        session.renew();
        return Ok(HttpResponse::Ok().json(user));
    } else {
        return Err(ApiError::new(401, "Credentials not valid!".to_string()));
    }
}

#[post("/signout")]
async fn signout(session: Session) -> Result<HttpResponse, ApiError> {
    let session_user_id: Option<Uuid> = session.get("user_id").expect("err");

    if let Some(_) = session_user_id {
        session.remove("user_id");
        return Ok(HttpResponse::Ok().json(json!({"message": "Signout success!"})));
    } else {
        return Err(ApiError::new(
            401,
            "Unauthorized: No user session found".to_string(),
        ));
    }
}

pub fn auth_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(signup)
            .service(signin)
            .service(signout),
    );
}
