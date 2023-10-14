use crate::app_state::AppState;
use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;

use crate::api;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(api::users::get_user))
        .route("/users/psn_code", put(api::users::update_psn_code))
        .route("/users/steam_id", put(api::users::update_steam_id))
        .route("/", get(api::api::root))
        .route("/index", get(api::api::index))
        .route("/health", get(api::api::healthcheck))
        .route("/auth/signup", post(api::auth::signup))
        .route("/auth/signin", post(api::auth::signin))
        .route("/auth/signout", post(api::auth::signout))
        .fallback(api::api::fallback)
        .with_state(state.db)
        .layer(TraceLayer::new_for_http())
}
