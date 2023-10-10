use actix_session::Session;
use actix_web::{
    post,
    web::{self, Json},
    HttpResponse,
};
use serde_json::json;
use uuid::Uuid;

use crate::models::users::{SignoutUserRequest, User, UserRequest};
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
            .insert(format!("{}", &user.id), &user.id)
            .expect("session insert error");
        session.renew();
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/signout")]
async fn singout(
    user: Json<SignoutUserRequest>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let id: Option<Uuid> = session.get(&user.id).expect("session get error");

    if let Some(_) = id {
        session.purge();
        Ok(HttpResponse::Ok().json(json!({"message": "Signout success!"})))
    } else {
        Err(ApiError::new(401, "Unauthorized".to_string()))
    }
}

pub fn auth_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(signup)
            .service(signin)
            .service(singout),
    );
}
