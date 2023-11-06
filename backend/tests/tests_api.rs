#[cfg(test)]
use ::axum_test::{TestServer, TestServerConfig};
use bb8_redis::{bb8, RedisConnectionManager};
use dotenv::dotenv;
use sea_orm::Database;
use webapp::{app_state::AppState, router::create_router};

async fn create_new_app() -> TestServer {
    dotenv().ok();

    let db = match Database::connect("postgresql://superuser:superpassword@localhost:5432/users")
        .await
    {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let redis_manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let redis_connection = bb8::Pool::builder().build(redis_manager).await.unwrap();

    let redis_pool = redis_connection.clone();
    let mut _conn = redis_pool.get().await.unwrap();

    let state = AppState {
        db,
        redis_connection,
    };

    let app = create_router(state);

    let config = TestServerConfig::builder().mock_transport().build();

    TestServer::new_with_config(app, config).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use serde::{Deserialize, Serialize};
    use webapp::models::response::Response;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct ErrorResponse {
        error: String,
    }

    #[tokio::test]
    async fn test_root_route_success() {
        let server = create_new_app();

        let response = server.await.get("/").await;

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.text(), "Hello, World!")
    }

    #[tokio::test]
    async fn test_not_found_route_success() {
        let server = create_new_app();

        let response = server.await.get("/root").await;

        assert_eq!(response.status_code(), StatusCode::NOT_FOUND);
        assert_eq!(response.text(), "Not Found")
    }

    #[tokio::test]
    async fn test_health_route_success() {
        let server = create_new_app();

        let response = server.await.get("/health").await;

        let test_response = Response {
            message: "Everything is working fine".to_string(),
        };

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.json::<Response>(), test_response)
    }

    #[tokio::test]
    async fn test_index_route_success() {
        let server = create_new_app();

        let response = server.await.get("/index").await;

        let test_response = Response {
            message: "use get and post on api/user".to_string(),
        };

        assert_eq!(response.status_code(), StatusCode::OK);
        assert_eq!(response.json::<Response>(), test_response)
    }

    #[tokio::test]
    async fn test_user_route_fail() {
        let server = create_new_app();

        let response = server.await.get("/users").await;

        let test_response = ErrorResponse {
            error: "Not authenticated!".to_string(),
        };

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.json::<ErrorResponse>(), test_response)
    }

    #[tokio::test]
    async fn test_users_psn_id_route_authorization_fails() {
        let server = create_new_app();

        let response = server.await.get("/users/psn_id").await;

        let test_response = ErrorResponse {
            error: "Not authenticated!".to_string(),
        };

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.json::<ErrorResponse>(), test_response)
    }

    #[tokio::test]
    async fn test_users_steam_id_route_authorization_fails() {
        let server = create_new_app();

        let response = server.await.get("/users/steam_id").await;

        let test_response = ErrorResponse {
            error: "Not authenticated!".to_string(),
        };

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.json::<ErrorResponse>(), test_response)
    }

    #[tokio::test]
    async fn test_users_xbox_id_route_authorization_fails() {
        let server = create_new_app();

        let response = server.await.get("/users/xbox_id").await;

        let test_response = ErrorResponse {
            error: "Not authenticated!".to_string(),
        };

        assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
        assert_eq!(response.json::<ErrorResponse>(), test_response)
    }
    //
    // #[tokio::test]
    // async fn test_signin_fails() {
    //     let server = create_new_app();
    //
    //     let response = server.await.
    //
    //     let test_response = ErrorResponse {
    //         error: "Not authenticated!".to_string(),
    //     };
    //
    //     assert_eq!(response.status_code(), StatusCode::UNAUTHORIZED);
    //     assert_eq!(response.json::<ErrorResponse>(), test_response)
    // }
}
