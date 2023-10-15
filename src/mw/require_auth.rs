// use axum::{
//     http::{HeaderMap, Request},
//     middleware::Next,
//     response::Response,
// };
// use hyper::StatusCode;
// use tower_cookies::Cookies;
// use tower_sessions::Session;

// use crate::utils::errors::ApiError;

// pub async fn require_auth<T>(
//     cookies: Cookies,
//     _headers: HeaderMap,
//     session: Session,
//     request: Request<T>,
//     next: Next<T>,
// ) -> Result<Response, ApiError> {
//     // let header_token = if let Some(token) = headers.get("x-auth-token") {
//     //     token.to_str().map_err(|error| {
//     //         eprintln!("Error extracting token from headers: {:?}", error);
//     //         ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
//     //     })?
//     // } else {
//     //     return Err(ApiError::new(
//     //         StatusCode::UNAUTHORIZED,
//     //         "not authenticated!",
//     //     ));
//     // };

//     let cookie = cookies.get("tower.sid").map(|c| c.value().to_string());



//     cookie.ok_or(ApiError::new(StatusCode::UNAUTHORIZED, "Not Authenticated!"))?;

//     Ok(next.run(request).await)
// }
