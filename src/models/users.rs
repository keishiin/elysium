use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset};

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub user_name: String,
    pub password: String,
    pub email: String,
    pub steam_id: Option<String>,
    pub psn_auth_code: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub id: String
}