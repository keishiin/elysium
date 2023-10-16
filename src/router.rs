use crate::{
    api::{
        api::{fallback, healthcheck, index, root},
        auth::{signin, signout, signup},
        users::{get_user, update_psn_code, update_steam_id},
    },
    app_state::AppState,
    mw::require_auth::require_auth,
};
use axum::{
    routing::{get, post, put},
    Router, middleware,
};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_user))
        .route("/users/psn_code", put(update_psn_code))
        .route("/users/steam_id", put(update_steam_id))
        .route("/auth/signout", post(signout))
        .route_layer(middleware::from_fn(require_auth))
        .route("/", get(root))
        .route("/index", get(index))
        .route("/health", get(healthcheck))
        .route("/auth/signup", post(signup))
        .route("/auth/signin", post(signin))
        .fallback(fallback)
        .with_state(state.db)
        .layer(TraceLayer::new_for_http())
}
