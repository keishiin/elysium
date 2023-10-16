use axum::{
    http::{HeaderMap, Request},
    middleware::Next,
    response::Response,
};
use hyper::{StatusCode, header::COOKIE};

use crate::utils::{errors::ApiError, jwt_auth_utils::validate_token, middware_utils::split_by_double_quotes};

pub async fn require_auth<T>(
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

    let value = split_by_double_quotes(header_token).ok_or_else(|| {
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid token format")
    })?;

    validate_token(value)?;

    Ok(next.run(request).await)
}