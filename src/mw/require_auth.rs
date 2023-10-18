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
            "Cookie is missing!",
        ));
    };

    let header_user_token = if let Some(user_token) = headers.get("axum-accountId") {
        user_token.to_str().map_err(|error| {
            eprintln!("Error extracting user token from headers: {:?}", error);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error user reading token",
            )
        })?
    } else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "axum-accountId is missing!",
        ));
    };

    let value = split_by_double_quotes(header_token.clone())
        .ok_or_else(|| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid token format"))?;
    eprintln!("cookie value: {:?}", header_token);

    let user_id = split_by_double_quotes(header_user_token)
        .ok_or_else(|| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid token format"))?;
    eprintln!("account id: {:?}", user_id.clone());

    validate_token(value.clone())?;

    let mut conn = redis_pool.get().await.unwrap();
    let reply: redis::Value = cmd("GET").arg(value).query_async(&mut *conn).await.unwrap();

    if reply == redis::Value::Nil {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "reply was nil"));
    } else if let redis::Value::Data(data) = reply {
        let reply_str = std::str::from_utf8(&data).unwrap(); // Convert the reply to a string
        eprintln!("reply_str was: {:?}", reply_str);
        if reply_str == user_id {
            Ok(next.run(request).await)
        } else {
            return Err(ApiError::new(
                StatusCode::UNAUTHORIZED,
                "first outer for stuff",
            ));
        }
    } else {
        // this should never be able to make it here hopefully
        unreachable!()
    }
}
