use actix_web::{web, App, HttpServer};

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let user_db = repository::database::Database::new();
    let app_data = web::Data::new(user_db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(api::api::index)
            .service(api::api::healthcheck)
            .default_service(web::route().to(api::api::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
