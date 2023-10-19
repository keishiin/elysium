use crate::utils::{
    errors::ApiError,
    jwt_auth_utils::validate_token,
    middware_utils::{get_header, split_by_double_quotes},
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
    let header_token = get_header(headers.clone(), COOKIE.to_string())?;
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;

    let value = split_by_double_quotes(header_token.as_str())?;
    let user_id = split_by_double_quotes(header_user_token.as_str())?;

    validate_token(&value.clone())?;

    let mut conn = redis_pool.get().await.unwrap();
    let reply: redis::Value = cmd("GET").arg(value).query_async(&mut *conn).await.unwrap();

    if reply == redis::Value::Nil {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "reply was nil"));
    } else if let redis::Value::Data(data) = reply {
        let reply_str = std::str::from_utf8(&data).unwrap(); // Convert the reply to a string
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
