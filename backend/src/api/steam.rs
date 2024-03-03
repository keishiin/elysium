use axum::{extract::{State, Query}, Json};
use hyper::{HeaderMap, StatusCode};
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use steam_api_wrapper::{services::get_owned_games, Steam};

use crate::{
    queries::users_q::get_user_by_id,
    utils::{errors::ApiError, middware_utils::get_header},
};

#[derive(Serialize, Deserialize)]
pub struct Cursor {
    cursor: Option<u32>
}

#[derive(Serialize, Deserialize)]
pub struct SteamOwnedGames {
    cursor: u32,
    data: Vec<get_owned_games::OwnedGame>,
}

pub async fn player_owned_games(
    State(db): State<DatabaseConnection>,
    State(api_key): State<String>,
    cursor: Query<Cursor>,
    headers: HeaderMap,
) -> Result<Json<SteamOwnedGames>, ApiError> {
    let header_user_token = get_header(headers, "axum-accountId".to_string())?;
    let user = get_user_by_id(&db, header_user_token.clone()).await?;

    let steam = Steam::new(api_key.as_str());
    let steam_id = user.steam_id;

    // this is highly digusting and needs to be updated asap in the db
    let steam_id_as_u64: Result<u64, ApiError> = steam_id
        .map(|id| {
            id.parse::<u64>()
                .map_err(|_| ApiError::new(StatusCode::BAD_REQUEST, "Failed to parse steam ID"))
        })
        .unwrap_or_else(|| Err(ApiError::new(StatusCode::BAD_REQUEST, "Missing steam id")));

    let response = steam
        .get_owned_games(steam_id_as_u64?, true, false)
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
        paginated_response = response.games[page as usize .. response.game_count as usize].to_vec();
    }
    else {
        paginated_response = response.games[page as usize .. max_cursor as usize].to_vec();
    }
    
    Ok(Json(SteamOwnedGames {
        cursor: max_cursor,
        data: paginated_response,
    }))
}

