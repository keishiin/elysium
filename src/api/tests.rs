#[cfg(test)]
mod tests {
    use crate::api;
    use crate::models::users::User;
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;
    use serde_json::to_string;

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

        if user.user_name != "" {
            let resp = User::delete_user_by_id(user.id).expect("failed to delete user");

            if resp == "deleted user" {
                let response = test::TestRequest::post()
                    .uri("/auth/signup")
                    .set_json(&request)
                    .send_request(&mut app)
                    .await;

                println!("Status: {:?}", response.status());
            }
        }
    }
}
