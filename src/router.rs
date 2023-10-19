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
    middleware,
    routing::{get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/users", get(get_user))
        .route("/users/psn_code", put(update_psn_code))
        .route("/users/steam_id", put(update_steam_id))
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .route("/index", get(index))
        .route("/health", get(healthcheck))
        .route("/auth/signout", post(signout))
        .route("/auth/signup", post(signup))
        .route("/auth/signin", post(signin))
        .fallback(fallback)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
}
