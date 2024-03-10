use super::errors::ApiError;
use hyper::StatusCode;

pub fn steam_id_to_u64(steam_id: Option<String>) -> Result<u64, ApiError> {
    return steam_id
        .map(|id| {
            id.parse::<u64>()
                .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Failed to parse steam ID"))
        })
        .unwrap_or_else(|| Err(ApiError::new(StatusCode::BAD_REQUEST, "Missing steam id")));
}
