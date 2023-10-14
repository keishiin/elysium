use axum::{extract::State, http::StatusCode, Json};

use crate::models::users::{
    ResponseUser, UserRequest, UserUpdateIdRequest, UserUpdatePsnCodeRequest,
};
use crate::queries::users_q::{
    get_user_by_id, get_user_by_username, update_psn_code_save, update_steam_id_save,
};
use crate::utils::errors::ApiError;
use crate::utils::hash::verify_password;
use sea_orm::DatabaseConnection;

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    user_info: Json<UserRequest>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_username(&db, user_info.username.clone()).await?;

    if !verify_password(&user_info.password, &user.password)? {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Incorrect username/password",
        ));
    }

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: user.psn_auth_code,
    }))
}

pub async fn update_steam_id(
    State(db): State<DatabaseConnection>,
    user_info: Json<UserUpdateIdRequest>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_id(&db, user_info.user_id.clone()).await?;

    update_steam_id_save(&db, user.clone(), user_info.steam_id.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: Some(user_info.steam_id.clone()),
        psn_auth_code: user.psn_auth_code,
    }))
}

pub async fn update_psn_code(
    State(db): State<DatabaseConnection>,
    user_info: Json<UserUpdatePsnCodeRequest>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_id(&db, user_info.user_id.clone()).await?;

    update_psn_code_save(&db, user.clone(), user_info.psn_code.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: Some(user_info.psn_code.clone()),
    }))
}
