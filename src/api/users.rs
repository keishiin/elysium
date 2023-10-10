use actix_session::Session;
use actix_web::{
    get, put,
    web::{self, Json},
    HttpResponse,
};
use uuid::Uuid;

use crate::models::users::{User, UserRequest, UserUpdateIdRequest, UserUpdatePsnCodeRequest};
use crate::utils::errors::ApiError;

#[get("/user")]
async fn get_user(user_id: Json<UserRequest>) -> HttpResponse {
    let user = User::get_user_by_id(&user_id.username);

    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_err) => HttpResponse::InternalServerError().body("User not found"),
    }
}

#[put("/update/steamId")]
async fn update_steam_id(
    user_req: Json<UserUpdateIdRequest>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let session_id: Option<Uuid> = session.get(&user_req.user_id).expect("not signed in");

    if let Some(_) = session_id {
        let user: User = User::update_steam_id(&user_req.user_id, &user_req.steam_id)
            .expect("and error occure trying to update");

        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

#[put("/update/psn_code")]
async fn update_psn_code(
    user_req: Json<UserUpdatePsnCodeRequest>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let session_id: Option<Uuid> = session.get(&user_req.user_id).expect("not signed in");

    if let Some(_) = session_id {
        let user: User = User::update_psn_code(&user_req.user_id, &user_req.psn_code)
            .expect("and error occure trying to update");

        Ok(HttpResponse::Ok().json(user))
    } else {
        Err(ApiError::new(401, "Credentials not valid!".to_string()))
    }
}

pub fn user_route_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_user)
            .service(update_steam_id)
            .service(update_psn_code),
    );
}
