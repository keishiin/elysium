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
            .configure(api::api::init_route_config)
            .configure(api::users::user_route_config)
            .configure(api::auth::auth_route_config)
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
