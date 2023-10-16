use axum::http::StatusCode;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::errors::ApiError;


#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    username: String,
}

pub fn create_token(username: String) -> Result<String, ApiError> {
    // add at least an hour for this timestamp
    let now = chrono::Utc::now();
    let expires_at = Duration::hours(1);
    let expires_at = now + expires_at;
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret("secretkey".as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn validate_token(token: &str) -> Result<bool, ApiError> {
    let key = DecodingKey::from_secret("secretkey".as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    eprintln!("Header_token: {:?}", token);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                ApiError::new(StatusCode::UNAUTHORIZED, "in: Validate not authenticated!")
            }
            _ => {
                eprintln!("Error validating token: {:?}", error);
                ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
            }
        })
        .map(|_claim| true)
}