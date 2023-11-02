use crate::{
    models::users::{User, UserRequest},
    queries::users_q::{create_user, get_user_by_username},
    utils::{
        errors::ApiError,
        hash::{hash_password, verify_password},
        jwt_auth_utils::create_token,
        middware_utils::get_header,
    },
};
use ::entity::users;
use axum::http::HeaderMap;
use axum::{extract::State, http::StatusCode, Json};
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use redis::cmd;
use sea_orm::{DatabaseConnection, Set};
use uuid::Uuid;

pub async fn signup(
    State(db): State<DatabaseConnection>,
    State(redis_pool): State<Pool<RedisConnectionManager>>,
    req_user: Json<User>,
) -> Result<HeaderMap, ApiError> {
    let new_user = users::ActiveModel {
        id: Set(Uuid::new_v4().to_string()),
        username: Set(req_user.username.clone()),
        password: Set(hash_password(&req_user.password)?),
        email: Set(req_user.email.clone()),
        ..Default::default()
    };

    let user = create_user(&db, new_user).await?;
    let session_key = format!("user_{}", user.id);

    let token = create_token(session_key)?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", token.parse().unwrap());
    headers.insert("axum-accountId", user.id.clone().parse().unwrap());

    let mut conn = redis_pool.get().await.unwrap();
    let _reply: redis::Value = cmd("SET")
        .arg(&[token, user.clone().id])
        .query_async(&mut *conn)
        .await
        .unwrap();

    Ok(headers)
}

pub async fn signin(
    State(db): State<DatabaseConnection>,
    State(redis_pool): State<Pool<RedisConnectionManager>>,
    user_info: Json<UserRequest>,
) -> Result<HeaderMap, ApiError> {
    let user = get_user_by_username(&db, user_info.username.clone()).await?;

    if !verify_password(&user_info.password, &user.password)? {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "Incorrect username/password",
        ));
    }
    let session_key = format!("user_{}", user.id);
    let token = create_token(session_key)?;

    let mut headers = HeaderMap::new();
    headers.insert("Authorization", token.parse().unwrap());
    headers.insert("axum-accountId", user.id.clone().parse().unwrap());

    let mut conn = redis_pool.get().await.unwrap();
    let _reply: redis::Value = cmd("SET")
        .arg(&[token, user.clone().id])
        .query_async(&mut *conn)
        .await
        .unwrap();

    Ok(headers)
}

pub async fn signout(
    State(redis_pool): State<Pool<RedisConnectionManager>>,
    headers: HeaderMap,
) -> Result<StatusCode, ApiError> {
    let header_token = get_header(headers, "Authorization".to_string())?;

    let mut conn = redis_pool.get().await.unwrap();
    let _reply: redis::Value = cmd("DEL")
        .arg(header_token)
        .query_async(&mut *conn)
        .await
        .unwrap();

    Ok(StatusCode::OK)
}
