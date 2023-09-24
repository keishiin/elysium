#[macro_use]
extern crate log;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use listenfd::ListenFd;
use std::env;

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    dotenv().ok();
    env_logger::init();

    let user_db = repository::database::Database::new();
    let app_data = web::Data::new(user_db);

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new( move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(api::api::index)
            .service(api::api::healthcheck)
            .default_service(web::route().to(api::api::not_found))
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("starting server");
    server.run().await

}
