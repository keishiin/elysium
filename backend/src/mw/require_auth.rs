use crate::utils::{
    errors::ApiError,
    jwt_auth_utils::validate_token,
    middware_utils::get_header,
};
use axum::{
    extract::State,
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use bb8_redis::{bb8::Pool, redis::cmd, RedisConnectionManager};
use hyper::StatusCode;

pub async fn require_auth<T>(
    State(redis_pool): State<Pool<RedisConnectionManager>>,
    headers: HeaderMap,
    request: Request<T>,
    next: Next<T>,
) -> Result<Response, ApiError> {
    let header_token = get_header(headers.clone(), "Authorization".to_string())?;
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;

    validate_token(&&header_token.clone())?;

    let mut conn = redis_pool.get().await.unwrap();
    let reply: redis::Value = cmd("GET").arg(header_token).query_async(&mut *conn).await.unwrap();

    if reply == redis::Value::Nil {
        return Err(ApiError::new(StatusCode::UNAUTHORIZED, "reply was nil"));
    } else if let redis::Value::Data(data) = reply {
        let reply_str = std::str::from_utf8(&data).unwrap();
        if reply_str == header_user_token {
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
