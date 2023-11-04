#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use webapp::{self, utils::errors::ApiError};

    #[test]
    fn hash_password_succeds() {
        let result = webapp::utils::hash::hash_password("test");
        assert!(result.is_ok());
    }

    // this needs to be re writen so it will infact fail porperly
    #[test]
    fn hash_password_fails() {
        let result = webapp::utils::hash::hash_password("");

        match result {
            Ok(_) => {}
            Err(error) => {
                assert_ne!(
                    error,
                    ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
                )
            }
        }
    }

    #[test]
    fn verify_password_succeeds() {
        let password_hash = webapp::utils::hash::hash_password("password").unwrap();
        let bool =
            webapp::utils::hash::verify_password("password", password_hash.as_str()).unwrap();
        assert_eq!(bool, true);
    }

    #[test]
    fn verify_password_fails_incorrect_password() {
        let password_hash = webapp::utils::hash::hash_password("password").unwrap();
        let result = webapp::utils::hash::verify_password("wrong", password_hash.as_str());

        match result {
            Ok(bool) => {
                assert_eq!(bool, false)
            }
            Err(error) => {
                assert_ne!(
                    error,
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "The was a problem verifying your password"
                    )
                )
            }
        }
    }

    #[test]
    fn verify_password_fails_incorrect_hash() {
        let result = webapp::utils::hash::verify_password("wrong", "wrong");

        match result {
            Ok(bool) => {
                assert_eq!(bool, false)
            }
            Err(error) => {
                assert_eq!(
                    error,
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "The was a problem verifying your password"
                    )
                )
            }
        }
    }
}
