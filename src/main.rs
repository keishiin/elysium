use actix_web::{get, web, App, HttpServer, HttpResponse, Responder, Result};
use serde::Serialize;

mod api;
mod models;
mod repository;


#[derive(Serialize)]
pub struct Response {
    pub message: String,
}

#[get("/")]
async fn index() -> impl Responder {
    let response = Response {
        message: "use get and post on api/user".to_string(),
    };
    HttpResponse::Ok().json(response)
}

#[get("/health")]
async fn healthcheck() -> impl Responder {
    let response = Response {
        message: "Everything is working fine".to_string(),
    };
    HttpResponse::Ok().json(response)
}


async fn not_found() -> Result<HttpResponse> {
    let response = Response {
        message: "Resource not found".to_string(),
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let user_db = repository::database::Database::new();
    let app_data = web::Data::new(user_db);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(api::api::config)
            .service(index)
            .service(healthcheck)
            .default_service(web::route().to(not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
