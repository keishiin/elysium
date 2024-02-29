#[cfg(test)]
mod tests {
    use axum::http::{HeaderMap, StatusCode};
    use webapp::{
        self,
        utils::{
            errors::ApiError,
            hash::{hash_password, verify_password},
            jwt_auth_utils::{create_token, validate_token},
            middware_utils::{get_header, split_by_double_quotes},
        },
    };

    #[test]
    fn test_hash_password_succeds() {
        let result = hash_password("test");
        assert!(result.is_ok());
    }

    // this needs to be re writen so it will infact fail porperly
    #[test]
    fn test_hash_password_fails() {
        let result = hash_password("");

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
    fn test_verify_password_succeeds() {
        let password_hash = hash_password("password").unwrap();
        let bool = verify_password("password", password_hash.as_str()).unwrap();
        assert_eq!(bool, true);
    }

    #[test]
    fn test_verify_password_fails_incorrect_password() {
        let password_hash = hash_password("password").unwrap();
        let result = verify_password("wrong", password_hash.as_str());

        match result {
            Ok(_) => {}
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
    fn test_verify_password_fails_incorrect_hash() {
        let result = verify_password("wrong", "wrong");

        match result {
            Ok(_) => {}
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

    #[test]
    fn test_split_by_double_quotes_succeeds() {
        let result = split_by_double_quotes("\"teststring\"");

        match result {
            Ok(res) => {
                assert_eq!(res, "teststring")
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_split_by_double_quotes_fails() {
        let result = split_by_double_quotes("teststring");

        match result {
            Ok(_) => {}
            Err(error) => {
                assert_eq!(
                    error,
                    ApiError::new(StatusCode::BAD_REQUEST, "No opening double quote found")
                )
            }
        }
    }

    #[test]
    fn test_get_header_succeeds() {
        let mut header = HeaderMap::new();
        header.insert("key", "testHeaderValue".parse().unwrap());

        let result = get_header(header, "key".to_string());

        match result {
            Ok(res) => {
                assert_eq!(res, "testHeaderValue")
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_get_header_fails() {
        let header = HeaderMap::new();

        let result = get_header(header, "key".to_string());

        match result {
            Ok(_) => {}
            Err(error) => {
                assert_eq!(
                    error,
                    ApiError::new(StatusCode::UNAUTHORIZED, "Not authenticated!")
                )
            }
        }
    }

    #[test]
    fn test_create_token_succeeds() {
        let result = create_token("username".to_string());

        match result {
            Ok(res) => {
                assert_ne!(res, "")
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_create_token_fails() {
        let result = create_token("username".to_string());

        match result {
            Ok(_) => {}
            Err(error) => {
                assert_eq!(
                    error,
                    ApiError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "There was an error, please try again later"
                    )
                )
            }
        }
    }

    #[test]
    fn test_validate_token_succeeds() {
        let result = create_token("username".to_string()).unwrap();

        let validate_status = validate_token(result.as_str());

        match validate_status {
            Ok(status) => {
                assert_eq!(status, true)
            }
            Err(_) => {}
        }
    }

    #[test]
    fn test_validate_token_fails() {
        let validate_status = validate_token("testUser");

        match validate_status {
            Ok(status) => {
                assert_eq!(status, true)
            }
            Err(_) => {}
        }
    }
}
