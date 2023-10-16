use crate::{
    models::users::{ResponseUser, User, UserRequest, UserSignOutRequest},
    queries::users_q::{create_user, get_user_by_username},
    utils::{
        errors::ApiError,
        hash::{hash_password, verify_password}, jwt_auth_utils::create_token,
    },
};
use ::entity::users;
use axum::debug_handler;
use axum::{extract::State, http::StatusCode, Json};
use axum::http::HeaderMap;
use sea_orm::{DatabaseConnection, Set};
use tower_sessions::Session;
use uuid::Uuid;

#[debug_handler]
pub async fn signup(
    State(db): State<DatabaseConnection>,
    _session: Session,
    req_user: Json<User>,
) -> Result<(HeaderMap, Json<ResponseUser>), ApiError> {
    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        user_name: Set(req_user.username.clone()),
        password: Set(hash_password(&req_user.password)?),
        email: Set(req_user.email.clone()),
        steam_id : Set(req_user.steam_id.clone()),
        psn_auth_code : Set(req_user.psn_auth_code.clone()),
        ..Default::default()
    };

    let user = create_user(&db, new_user).await?;
    let session_key = format!("user_{}", user.id);

    let token = create_token(session_key)?;
    let completed_token = format!("s.id={:?}", token);

    let mut headers = HeaderMap::new();
    headers.insert("set-cookie", completed_token.parse().unwrap());

    let return_user = ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: user.psn_auth_code,
    };

    Ok((headers, Json(return_user)))
}

#[debug_handler]
pub async fn signin(
    State(db): State<DatabaseConnection>,
    _session: Session,
    user_info: Json<UserRequest>,
) -> Result<(HeaderMap, Json<ResponseUser>), ApiError> {
    let user = get_user_by_username(&db, user_info.username.clone()).await?;

    if !verify_password(&user_info.password, &user.password)? {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Incorrect username/password",
        ));
    }
    let session_key = format!("user_{}", user.id);
    let token = create_token(session_key)?;
    let completed_token = format!("s.id={:?}", token);

    let mut headers = HeaderMap::new();
    headers.insert("set-cookie", completed_token.parse().unwrap());

    let return_user = ResponseUser {
        id: user.id,
        username: user.user_name,
        email: user.email,
        steam_id: user.steam_id,
        psn_auth_code: user.psn_auth_code,
    };

    Ok((headers, Json(return_user)))
}

#[debug_handler]
pub async fn signout(
    session: Session,
    user_req: Json<UserSignOutRequest>,
) -> Result<StatusCode, ApiError> {

    let user_id = &user_req.user_id;
    let session_key = format!("user_{}", &user_id);
    let value: Option<usize> = session.remove(&session_key).unwrap_or_default();

    match value {
        Some(_) => Ok(StatusCode::INTERNAL_SERVER_ERROR),
        None => Ok(StatusCode::OK)
    }

}
