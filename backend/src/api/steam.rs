use axum::{
    extract::{Query, State},
    Json,
};
use hyper::HeaderMap;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use steam_api_wrapper::{
    services::{get_owned_games, get_player_info, get_recently_played_games},
    Steam,
};

use crate::{
    queries::users_q::get_user_by_id,
    utils::{api_utils::steam_id_to_u64, errors::ApiError, middware_utils::get_header},
};

#[derive(Serialize, Deserialize)]
pub struct Cursor {
    cursor: Option<u32>,
}

#[derive(Serialize, Deserialize)]
pub struct SteamOwnedGames {
    cursor: u32,
    data: Vec<get_owned_games::OwnedGame>,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerSummary {
    response: Vec<get_player_info::Player>,
}

#[derive(Serialize, Deserialize)]
pub struct RecentlyPlayedGames {
    response: get_recently_played_games::Games,
}

pub async fn player_owned_games(
    State(db): State<DatabaseConnection>,
    State(api_key): State<String>,
    cursor: Query<Cursor>,
    headers: HeaderMap,
) -> Result<Json<SteamOwnedGames>, ApiError> {
    let user_id = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, user_id).await?;

    let steam = Steam::new(api_key.as_str());

    let response = steam
        .get_owned_games(steam_id_to_u64(user.steam_id)?, true, false)
        .await
        .unwrap();

    let page: u32;

    if let Some(cursor) = cursor.cursor {
        page = cursor;
    } else {
        page = 0;
    }

    let max_cursor = page + 10;
    let paginated_response;

    if response.game_count < max_cursor {
        paginated_response = response.games[page as usize..response.game_count as usize].to_vec();
    } else {
        paginated_response = response.games[page as usize..max_cursor as usize].to_vec();
    }

    Ok(Json(SteamOwnedGames {
        cursor: max_cursor,
        data: paginated_response,
    }))
}

pub async fn get_player_recently_played_games(
    State(db): State<DatabaseConnection>,
    State(api_key): State<String>,
    headers: HeaderMap,
) -> Result<Json<RecentlyPlayedGames>, ApiError> {
    let user_id = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, user_id).await?;

    let steam = Steam::new(api_key.as_str());

    let response = steam
        .get_recently_played_games(steam_id_to_u64(user.steam_id)?)
        .await
        .unwrap();

    Ok(Json(RecentlyPlayedGames { response }))
}

pub async fn player_summary(
    State(db): State<DatabaseConnection>,
    State(api_key): State<String>,
    headers: HeaderMap,
) -> Result<Json<PlayerSummary>, ApiError> {
    let user_id = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, user_id).await?;

    let steam = Steam::new(api_key.as_str());

    let response = steam
        .get_player_summaries(steam_id_to_u64(user.steam_id)?)
        .await
        .unwrap();

    Ok(Json(PlayerSummary { response }))
}
