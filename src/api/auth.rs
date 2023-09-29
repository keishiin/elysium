use actix_web::web;
use actix_session::Session;
use uuid::Uuid;
use serde_json::json;
use actix_web::{get, post, put, web::Json, HttpResponse};

use crate::models::users::{User, UserRequest, UserUpdateIdRequest, SignoutUserRequest, UserUpdatePsnCodeRequest};
use crate::utils::errors::ApiError;

#[get("/user")]
async fn get_user(user_id: Json<UserRequest>) -> HttpResponse {
    let user = User::get_user_by_id(&user_id.username);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_err) => HttpResponse::InternalServerError().body("User not found")
    }
}

#[put("/update/steamId")]
async fn update_steam_id(user_req: Json<UserUpdateIdRequest>, session: Session) -> Result<HttpResponse, ApiError> {

    let session_id: Option<Uuid> = session.get(&user_req.user_id).expect("not signed in");

    if let Some(_) = session_id {
        let user: User = User::update_steam_id(&user_req.user_id, &user_req.steam_id).expect("and error occure trying to update");
    
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[put("/update/psn_code")]
async fn update_psn_code(user_req: Json<UserUpdatePsnCodeRequest>, session: Session) -> Result<HttpResponse, ApiError> {

    let session_id: Option<Uuid> = session.get(&user_req.user_id).expect("not signed in");

    if let Some(_) = session_id {
        let user: User = User::update_psn_code(&user_req.user_id, &user_req.psn_code).expect("and error occure trying to update");
    
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
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
        session.insert(format!("{}", &user.id), &user.id).expect("session insert error");
        session.renew();
        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[post("/signout")]
async fn singout(user: Json<SignoutUserRequest>, session: Session) -> Result<HttpResponse, ApiError> {

    let id: Option<Uuid> = session.get(&user.id).expect("session get error");

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
        .service(singout)
        .service(update_steam_id)
        .service(update_psn_code)
    );
}