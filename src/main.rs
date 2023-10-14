use axum::{
    routing::{get, post, put},
    Router,
};

use dotenv::dotenv;
use sea_orm::Database;
use sea_orm::DatabaseConnection;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod models;
mod queries;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_testing=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = match Database::connect(db_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };
    let state = AppState { db };

    let app = Router::new()
        .route("/", get(api::api::root))
        .route("/index", get(api::api::index))
        .route("/health", get(api::api::healthcheck))
        .route("/auth/signup", post(api::auth::signup))
        .route("/auth/signin", post(api::auth::signin))
        .route("/auth/signout", post(api::auth::signout))
        .route("/users", get(api::users::get_user))
        .route("/users/psn_code", put(api::users::update_psn_code))
        .route("/users/steam_id", put(api::users::update_steam_id))
        .fallback(api::api::fallback)
        .with_state(state.db.clone())
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
