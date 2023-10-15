use std::net::SocketAddr;

use app_state::AppState;
use axum::{error_handling::HandleErrorLayer, BoxError};
use hyper::StatusCode;
use time::Duration;
use tower::ServiceBuilder;
use tower_sessions::{fred::prelude::*, RedisStore, SessionManagerLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::router::create_router;

mod api;
pub mod app_state;
mod models;
mod mw;
mod queries;
mod router;
pub mod utils;

pub async fn run(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_testing=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let client = RedisClient::default();
    let redis_conn = client.connect();

    let session_store = RedisStore::new(client);
    let session_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|_: BoxError| async {
            StatusCode::BAD_REQUEST
        }))
        .layer(
            SessionManagerLayer::new(session_store)
                .with_secure(false)
                .with_name("axum-session")
        );

    let router = create_router(state);
    let app = router.layer(session_service);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    redis_conn.await??;

    Ok(())
}
