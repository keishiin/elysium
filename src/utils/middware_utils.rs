use axum::http::{HeaderMap, StatusCode};

use super::errors::ApiError;

pub fn split_by_double_quotes(input: &str) -> Option<&str> {
    let start = input.find('"')? + 1;
    let end = input.rfind('"')?;
    Some(&input[start..end])
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
