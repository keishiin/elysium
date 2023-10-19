use axum::{extract::State, Json};

use crate::models::users::{ResponseUser, UserRequestsWithAuth};
use crate::queries::users_q::{
    get_user_by_id, get_user_by_username, update_psn_code_save, update_steam_id_save,
};
use crate::utils::errors::ApiError;
use sea_orm::DatabaseConnection;

pub async fn get_user(
    State(db): State<DatabaseConnection>,
    user_info: Json<UserRequestsWithAuth>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_username(&db, user_info.username.clone()).await?;

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
    user_info: Json<UserRequestsWithAuth>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_id(&db, user_info.user_id.clone()).await?;
    let steam_id = user_info.steam_id.clone().unwrap_or_default();

    update_steam_id_save(&db, user.clone(), steam_id.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user_info.steam_id.clone(),
        psn_auth_code: user.psn_auth_code,
    }))
}

pub async fn update_psn_code(
    State(db): State<DatabaseConnection>,
    user_info: Json<UserRequestsWithAuth>,
) -> Result<Json<ResponseUser>, ApiError> {
    let user = get_user_by_id(&db, user_info.user_id.clone()).await?;
    let psn_code = user_info.psn_code.clone().unwrap_or_default();
    update_psn_code_save(&db, user.clone(), psn_code.clone()).await?;

    Ok(Json(ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: user_info.psn_code.clone(),
    }))
}
