use axum::{extract::State, Json};
use hyper::HeaderMap;

use crate::models::users::{ResponseUser, UserRequestsWithAuth};
use crate::queries::users_q::{
    get_user_by_id, update_psn_id_save, update_steam_id_save, update_xbox_id_save,
};
use crate::utils::errors::ApiError;
use crate::utils::middware_utils::get_header;
use sea_orm::DatabaseConnection;

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    headers: HeaderMap,
) -> Result<Json<ResponseUser>, ApiError> {
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, header_user_token.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
        email: user.email,
        steam_id: user.steam_id,
        psn_id: user.psn_id,
        xbox_id: user.xbox_id,
    }))
}

pub async fn update_steam_id(
    State(db): State<DatabaseConnection>,
    headers: HeaderMap,
    user_info: Json<UserRequestsWithAuth>,
) -> Result<Json<ResponseUser>, ApiError> {
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, header_user_token.clone()).await?;
    let steam_id = user_info.steam_id.clone().unwrap_or_default();

    update_steam_id_save(&db, user.clone(), steam_id.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
        email: user.email,
        steam_id: user_info.steam_id.clone(),
        psn_id: user.psn_id,
        xbox_id: user.xbox_id,
    }))
}

pub async fn update_psn_id(
    State(db): State<DatabaseConnection>,
    headers: HeaderMap,
    user_info: Json<UserRequestsWithAuth>,
) -> Result<Json<ResponseUser>, ApiError> {
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, header_user_token.clone()).await?;
    let psn_code = user_info.psn_id.clone().unwrap_or_default();
    update_psn_id_save(&db, user.clone(), psn_code.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
        email: user.email,
        steam_id: user.steam_id,
        psn_id: user_info.psn_id.clone(),
        xbox_id: user.xbox_id,
    }))
}

pub async fn update_xbox_id(
    State(db): State<DatabaseConnection>,
    headers: HeaderMap,
    user_info: Json<UserRequestsWithAuth>,
) -> Result<Json<ResponseUser>, ApiError> {
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, header_user_token.clone()).await?;
    let xbox_id = user_info.xbox_id.clone().unwrap_or_default();
    update_xbox_id_save(&db, user.clone(), xbox_id.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.username,
        email: user.email,
        steam_id: user.steam_id,
        psn_id: user.psn_id,
        xbox_id: user_info.xbox_id.clone(),
    }))
}
