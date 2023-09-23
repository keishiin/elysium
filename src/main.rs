use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::Arc;

struct AppState {
    users: Mutex<HashMap<u32, User>>,
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    user_name: String,
    password: String,
    email: String,
}


#[derive(Serialize, Deserialize)]
struct UserRequest {
    id: u32
}


#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("use get and post on /user")
}

#[get("/user")]
async fn get_user(id: web::Json<UserRequest>, data: web::Data<AppState>) -> impl Responder {
    let users = data.users.lock().unwrap();
    match users.get(&id.id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().json("user not found"),
    }
}

#[post("/user")]
async fn create_user(user: web::Json<User>, data: web::Data<AppState>) -> impl Responder {
    let mut users = data.users.lock().unwrap();
    users.insert(user.id, user.into_inner());
    HttpResponse::Ok().json("created user")
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let shared_data = web::Data::new(AppState {
        users: Mutex::new(HashMap::new()), 
    });

    HttpServer::new(move || {
        App::new()
            .app_data(shared_data.clone())
            .service(index)
            .service(get_user)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
