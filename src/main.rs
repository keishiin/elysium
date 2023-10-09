#[macro_use]
extern crate log;

use actix_session::{storage::RedisActorSessionStore, SessionMiddleware};
use actix_web::{middleware, web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod api;
mod models;
mod repository;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let redis_port = env::var("REDIS_PORT").expect("Redis port not set");
    let redis_host = env::var("HOST").expect("Redis host not set");

    let user_db = repository::database::Database::new();
    let app_data = web::Data::new(user_db);

    let private_key = actix_web::cookie::Key::generate();

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(
                SessionMiddleware::builder(
                    RedisActorSessionStore::new(format!("{}:{}", redis_host, redis_port)),
                    private_key.clone(),
                )
                .build(),
            )
            .wrap(middleware::Logger::default())
            .app_data(app_data.clone())
            .configure(api::api::config)
            .configure(api::auth::auth_config)
            .default_service(web::route().to(api::api::not_found))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("SERVER_PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    log::info!("starting HTTP server at http://localhost:8080");
    server.run().await
}

#[cfg(test)]
mod tests {
    use crate::api;
    use actix_web::{http::StatusCode, test, web, App};
    use serde_json::json;
    use serde_json::to_string;

    #[actix_web::test]
    async fn test_index_get_status() {
        let mut app = test::init_service(App::new().configure(api::api::config)).await;

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
        let mut app = test::init_service(App::new().configure(api::api::config)).await;

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
    async fn test_does_not_exist() {
        let mut app = test::init_service(
            App::new()
                .configure(api::api::config)
                .default_service(web::route().to(api::api::not_found)),
        )
        .await;

        let resp_not_found = test::TestRequest::get()
            .uri("/")
            .send_request(&mut app)
            .await;

        let resp_found = test::TestRequest::get()
            .uri("/api/")
            .send_request(&mut app)
            .await;

        assert_eq!(resp_not_found.status(), StatusCode::NOT_FOUND);
        assert_ne!(resp_found.status(), StatusCode::NOT_FOUND);
        assert!(resp_found.status().is_success());
    }
}

