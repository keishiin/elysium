use axum::http::{HeaderMap, StatusCode};

use super::errors::ApiError;

pub fn split_by_double_quotes(input: &str) -> Result<String, ApiError> {
    let start = input.find('"').ok_or(ApiError::new(
        StatusCode::BAD_REQUEST,
        "No opening double quote found",
    ))? + 1;
    let end = input.rfind('"').ok_or(ApiError::new(
        StatusCode::BAD_REQUEST,
        "No closing double quote found",
    ))?;

    let result = input[start..end].to_string();
    Ok(result)
}

pub fn get_header(headers: HeaderMap, key: String) -> Result<String, ApiError> {
    let header_token = if let Some(token) = headers.clone().get(key) {
        token
            .to_str()
            .map_err(|error| {
                eprintln!("Error extracting token from headers: {:?}", error);
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
            })?
            .to_string()
    } else {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "not authenticated!",
        ));
    };
    Ok(header_token)
}
