use std::fmt::Error;
use serde::{Deserialize, Serialize};
use diesel::{Queryable, Insertable, AsChangeset, QueryDsl, RunQueryDsl};
use chrono::prelude::*;

use crate::repository::database::Database;
use crate::repository::schema::users::dsl::*;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub user_name: String,
    // #[serde(skip_serializing)]
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

impl User {

    pub fn get_user_by_id(user_id: &str) -> Option<User> {
        let conn = Database::new();

        let user = users
            .find(user_id)
            .get_result::<User>(&mut conn.pool.get().unwrap())
            .expect("Expect loading user by id");

        Some(user)
    }

    pub fn create_user(user: User) -> Result<User, Error> {
        let conn = Database::new();

        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..user
        };
        
        diesel::insert_into(users)
            .values(&user)
            .execute(&mut  conn.pool.get().unwrap())
            .expect("Error creating new user");

        Ok(user)
    }
}
