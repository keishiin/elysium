use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub steam_id: Option<String>,
    pub psn_auth_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    #[serde(default)]
    pub id: String,
    pub username: String,
    pub email: String,
    pub steam_id: Option<String>,
    pub psn_auth_code: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequestsWithAuth {
    pub user_id: String,
    pub username: String,
    pub steam_id: Option<String>,
    pub psn_code: Option<String>,
}
