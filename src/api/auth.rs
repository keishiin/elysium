use ::entity::users;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

const COOKIE_NAME: &str = "x-auth-token";

use crate::{
    models::users::{ResponseUser, User, UserRequest, UserSignOutRequest},
    queries::users_q::{create_user, get_user_by_id, get_user_by_username},
    utils::{
        errors::ApiError,
        hash::{hash_password, verify_password},
    },
};
use tower_cookies::{Cookies, Cookie};
use axum::debug_handler;

#[debug_handler]
pub async fn signup(
    State(db): State<DatabaseConnection>,
    cookies: Cookies,
    req_user: Json<User>,
) -> Result<Json<ResponseUser>, ApiError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };

    new_user.id = Set(Uuid::new_v4().to_string());
    new_user.user_name = Set(req_user.username.clone());
    new_user.password = Set(hash_password(&req_user.password)?);
    new_user.email = Set(req_user.email.clone());
    new_user.steam_id = Set(req_user.steam_id.clone());
    new_user.psn_auth_code = Set(req_user.psn_auth_code.clone());

    let user = create_user(&db, new_user).await?;

    cookies.add(Cookie::new(COOKIE_NAME, "testing"));

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: user.psn_auth_code,
    }))
}

pub async fn signin(
    State(db): State<DatabaseConnection>,
    cookies: Cookies,
    user_info: Json<UserRequest>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_username(&db, user_info.username.clone()).await?;

    if !verify_password(&user_info.password, &user.password)? {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Incorrect username/password",
        ));
    }

    cookies.add(Cookie::new(COOKIE_NAME, "testing"));

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: user.psn_auth_code,
    }))
}

pub async fn signout(
    State(db): State<DatabaseConnection>,
    cookies: Cookies,
    user_req: Json<UserSignOutRequest>,
) -> Result<StatusCode, ApiError> {
    let user = get_user_by_id(&db, user_req.user_id.clone()).await?;

    if !verify_password(&user_req.password, &user.password)? {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Incorrect username/password",
        ));
    }

    cookies.remove(Cookie::new(COOKIE_NAME, ""));

    Ok(StatusCode::OK)
}
