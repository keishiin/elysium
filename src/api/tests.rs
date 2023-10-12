#[cfg(test)]
mod tests {
    use crate::api;
    use crate::models::users::User;
    use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
    use actix_web::{
        http::{header, StatusCode},
        test, web, App,
    };
    use dotenv::dotenv;
    use serde_json::json;
    use serde_json::to_string;
    use std::env;

    #[actix_web::test]
    async fn test_index_get_status() {
        let mut app = test::init_service(App::new().configure(api::api::init_route_config)).await;

        let resp = test::TestRequest::get()
            .uri("/api/")
            .send_request(&mut app)
            .await;

        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_get_response() {
        let response = json! ({
            "message": "use get and post on api/user".to_string(),
        });
        let mut app = test::init_service(App::new().configure(api::api::init_route_config)).await;

        let resp = test::TestRequest::get()
            .uri("/api/")
            .send_request(&mut app)
            .await;

        let response_str = to_string(&response).expect("Failed to serialize JSON");

        let body_bytes = test::read_body(resp).await;

        let body_str = String::from_utf8_lossy(&body_bytes);

        assert_eq!(body_str, response_str);
    }

    #[actix_web::test]
    async fn test_url_does_not_exist() {
        let mut app = test::init_service(
            App::new()
                .configure(api::api::init_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let resp_not_found = test::TestRequest::get()
            .uri("/")
            .send_request(&mut app)
            .await;

        assert_eq!(resp_not_found.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_url_does_exist_for_not_found() {
        let mut app = test::init_service(
            App::new()
                .configure(api::api::init_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let resp_found = test::TestRequest::get()
            .uri("/api/")
            .send_request(&mut app)
            .await;

        assert_ne!(resp_found.status(), StatusCode::NOT_FOUND);
        assert!(resp_found.status().is_success());
    }

    #[actix_web::test]
    async fn test_user_creation() {
        let mut app = test::init_service(
            App::new()
                .configure(api::auth::auth_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let username = "testUser".to_string();
        let request = json!({
            "user_name": "testUser",
            "password": "test123",
            "email": "testUser@test.com",
            "steam_id": null,
            "psn_auth_code": null
        });

        let user: User = User::get_user_by_username(&username).expect("failed to find user");
        let _resp = User::delete_user_by_id(user.id).expect("failed to delete user");

        let response = test::TestRequest::post()
            .uri("/auth/signup")
            .set_json(&request)
            .send_request(&mut app)
            .await;

        assert!(response.status().is_success());
    }

    #[actix_web::test]
    async fn test_user_creation_failed() {
        let mut app = test::init_service(
            App::new()
                .configure(api::auth::auth_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let request = json!({
            "user_name": "pwd",
            "password": "test123",
            "email": "testUser@test.com",
            "steam_id": null,
            "psn_auth_code": null
        });

        let response = test::TestRequest::post()
            .uri("/auth/signup")
            .set_json(&request)
            .send_request(&mut app)
            .await;

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[actix_web::test]
    async fn test_user_signin_failed() {
        let mut app = test::init_service(
            App::new()
                .configure(api::auth::auth_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let signin_request = json!({
            "username": "pwd",
            "password": "test12sdfsfsdfds3",
        });

        let signin_response = test::TestRequest::post()
            .uri("/auth/signin")
            .set_json(&signin_request)
            .send_request(&mut app)
            .await;

        assert_eq!(signin_response.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_user_signin_success() {
        let mut app = test::init_service(
            App::new()
                .configure(api::auth::auth_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let signin_request = json!({
            "username": "pwd",
            "password": "test123",
        });

        let signin_response = test::TestRequest::post()
            .uri("/auth/signin")
            .set_json(&signin_request)
            .send_request(&mut app)
            .await;

        assert_eq!(signin_response.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_user_signout_failed() {
        let mut app = test::init_service(
            App::new()
                .configure(api::auth::auth_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let response = test::TestRequest::post()
            .uri("/auth/signout")
            .send_request(&mut app)
            .await;

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_user_signout_success() {
        dotenv().ok();
        env_logger::init();

        let redis_port = env::var("REDIS_PORT").expect("Redis port not set");
        let redis_host = env::var("HOST").expect("Redis host not set");

        let private_key = actix_web::cookie::Key::generate();

        let mut app = test::init_service(
            App::new()
                .wrap(
                    SessionMiddleware::builder(
                        RedisActorSessionStore::new(format!("{}:{}", redis_host, redis_port)),
                        private_key.clone(),
                    )
                    .build(),
                )
                .configure(api::auth::auth_route_config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let signin_request = json!({
            "username": "test1234",
            "password": "test123",
        });

        let signin_response = test::TestRequest::post()
            .uri("/auth/signin")
            .set_json(&signin_request)
            .send_request(&mut app)
            .await;

        let head = signin_response.headers();

        if let Some(c) = head.get(header::SET_COOKIE) {
            let cookie = c.to_str().unwrap_or("header not in utf8");
            let parts: Vec<&str> = cookie.split(";").collect();
            let id = parts.iter().find(|&&part| part.trim().starts_with("id="));

            match id {
                Some(id) => {
                    let id = id.trim_start_matches("id=").trim();
                    let signout_response = test::TestRequest::post()
                        .uri("/auth/signout")
                        .insert_header(("Cookie", format!("id={};", id)))
                        .send_request(&mut app)
                        .await;

                    assert_eq!(signout_response.status(), StatusCode::OK);
                }
                None => {
                    // if it ever reaches here, the test should fail
                    assert_eq!(1, 2);
                }
            }
        } else {
            // if it ever reaches here, the test should also fail
            assert_eq!(2, 4);
        }
    }
}
