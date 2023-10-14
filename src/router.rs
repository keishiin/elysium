use crate::{app_state::AppState, api::{
    auth::{signup, signin, signout}, 
    api::{healthcheck, index, root, fallback}, 
    users::{get_user, update_psn_code, update_steam_id}
}};
use axum::{
    routing::{get, post, put},
    Router,
};
use tower_http::trace::TraceLayer;
use tower_cookies::CookieManagerLayer;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(get_user))
        .route("/users/psn_code", put(update_psn_code))
        .route("/users/steam_id", put(update_steam_id))
        .route("/auth/signout", post(signout))
        .route("/", get(root))
        .route("/index", get(index))
        .route("/health", get(healthcheck))
        .route("/auth/signup", post(signup))
        .route("/auth/signin", post(signin))
        .fallback(fallback)
        .with_state(state.db)
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
}
