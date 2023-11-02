use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(default)]
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub email: String,
    pub steam_id: Option<String>,
    pub psn_id: Option<String>,
    pub xbox_id: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequestsWithAuth {
    pub steam_id: Option<String>,
    pub psn_id: Option<String>,
    pub xbox_id: Option<String>,
}
