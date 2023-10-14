use axum::{http::StatusCode, Json};

use crate::models::response::Response;

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn index() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "use get and post on api/user".to_string(),
    };
    return (StatusCode::OK, Json(response));
}

pub async fn healthcheck() -> (StatusCode, Json<Response>) {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    return (StatusCode::OK, Json(response));
}

pub async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}
