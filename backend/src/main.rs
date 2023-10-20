use bb8_redis::{bb8, RedisConnectionManager};
use dotenv::dotenv;
use redis::cmd;
use sea_orm::Database;
use webapp::{app_state::AppState, run};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let redis_url = std::env::var("REDIS_URL").expect("No host set for redis");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    eprintln!("db url: {:?}", db_url);
    eprintln!("redis url: {:?}", redis_url);

    let db = match Database::connect(db_url).await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("Error connecting to the database: {:?}", error);
            panic!();
        }
    };

    let redis_manager = RedisConnectionManager::new(redis_url).unwrap();
    let redis_connection = bb8::Pool::builder().build(redis_manager).await.unwrap();

    let redis_pool = redis_connection.clone();
    let mut conn = redis_pool.get().await.unwrap();
    let reply: String = cmd("PING").query_async(&mut *conn).await.unwrap();

    eprintln!("Redis connection PING response: {:?}", reply);
    let state = AppState {
        db,
        redis_connection,
    };

    let _ = run(state).await;
}
