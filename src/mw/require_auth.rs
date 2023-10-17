use crate::utils::{
    errors::ApiError, jwt_auth_utils::validate_token, middware_utils::split_by_double_quotes,
};
use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use bb8_redis::{bb8::Pool, redis::cmd, RedisConnectionManager};
use hyper::{header::COOKIE, StatusCode};

pub async fn require_auth<T>(
    State(redis_pool): State<Pool<RedisConnectionManager>>,
    headers: HeaderMap,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, ApiError> {
    let header_token = if let Some(token) = headers.get(COOKIE) {
        token.to_str().map_err(|error| {
            eprintln!("Error extracting token from headers: {:?}", error);
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?
    } else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ));
    };

    let value = split_by_double_quotes(header_token)
        .ok_or_else(|| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid token format"))?;

    validate_token(value)?;

    let mut conn = redis_pool.get().await.unwrap();
    let reply: String = cmd("PONG").query_async(&mut *conn).await.unwrap();

    eprintln!("Redis connection PING response: {:?}", reply);

    Ok(next.run(request).await)
}

