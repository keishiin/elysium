use crate::{
    api::{
        api::{fallback, healthcheck, index, root},
        auth::{signin, signout, signup},
        steam::{
            get_game_achievments_schema, get_player_ahcievements_for_game,
            get_player_recently_played_games, player_owned_games, player_summary,
        },
        users::{get_user, update_psn_id, update_steam_id, update_xbox_id},
    },
    app_state::AppState,
    mw::require_auth::require_auth,
};
use axum::{
    middleware,
    routing::{get, post, put},
    Router,
};
use hyper::{
    header::{AUTHORIZATION, CONTENT_TYPE},
    Method,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

pub fn create_router(state: AppState) -> Router {
    let cors: CorsLayer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT])
        .allow_origin([
            "http://127.0.0.1:3000".parse().unwrap(),
            "http://localhost:3000".parse().unwrap(),
        ])
        .allow_headers([
            AUTHORIZATION,
            CONTENT_TYPE,
            "axum-accountId".parse().unwrap(),
            "axum-appid".parse().unwrap(),
        ])
        .expose_headers([
            "authorization".parse().unwrap(),
            "axum-accountId".parse().unwrap(),
            "axum-appid".parse().unwrap(),
        ])
        .allow_credentials(true);

    Router::new()
        .route("/users", get(get_user))
        .route("/users/psn_id", put(update_psn_id))
        .route("/users/xbox_id", put(update_xbox_id))
        .route("/users/steam_id", put(update_steam_id))
        .route("/steam/games", get(player_owned_games))
        .route("/steam/player-profile", get(player_summary))
        .route("/steam/game-schema", get(get_game_achievments_schema))
        .route(
            "/steam/game-achievements",
            get(get_player_ahcievements_for_game),
        )
        .route(
            "/steam/recently-played",
            get(get_player_recently_played_games),
        )
        .route_layer(middleware::from_fn_with_state(state.clone(), require_auth))
        .route("/auth/signout", post(signout))
        .route("/auth/signup", post(signup))
        .route("/auth/signin", post(signin))
        .route("/index", get(index))
        .route("/health", get(healthcheck))
        .route("/", get(root))
        .fallback(fallback)
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
