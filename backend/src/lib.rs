use std::net::SocketAddr;

use app_state::AppState;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::router::create_router;

pub mod api;
pub mod app_state;
pub mod models;
pub mod mw;
pub mod queries;
pub mod router;
pub mod utils;

pub async fn run(state: AppState) -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_testing=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let router = create_router(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::debug!("listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    // axum::Server::bind(&addr)
    //     .serve(router.into_make_service())
    //     .await?;

    Ok(())
}
