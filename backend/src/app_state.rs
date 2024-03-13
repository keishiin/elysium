use axum::extract::FromRef;
use bb8::Pool;
use bb8_redis::RedisConnectionManager;
use sea_orm::DatabaseConnection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db: DatabaseConnection,
    pub redis_connection: Pool<RedisConnectionManager>,
    pub steam_api_key: String,
}
