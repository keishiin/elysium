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

    let value = split_by_double_quotes(header_token.clone())
        .ok_or_else(|| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid token format"))?;

    validate_token(value.clone())?;

    let mut conn = redis_pool.get().await.unwrap();
    let reply: redis::Value = cmd("GET").arg(value).query_async(&mut *conn).await.unwrap();

    eprintln!("Redis connection response was: {:?}", reply);
    let user_id = "3cf620ec-6ee9-4f91-861a-8fbc15e3361f";

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
