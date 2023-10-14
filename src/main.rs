use dotenv::dotenv;
use sea_orm::Database;
use webapp::{app_state::AppState, run};

#[tokio::main]
async fn main() {
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

    let _ = run(state).await;
}
