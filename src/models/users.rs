use argon2::Config;
use chrono::prelude::*;
use diesel::prelude::*;
use diesel::{AsChangeset, Insertable, QueryDsl, Queryable, RunQueryDsl};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Error;

use crate::repository::database::Database;
use crate::repository::schema::users::dsl::*;
use crate::utils::errors::ApiError;

#[derive(Serialize, Deserialize, Debug, Clone, Queryable, Insertable, AsChangeset)]
#[diesel(table_name = crate::repository::schema::users)]
pub struct User {
    #[serde(default)]
    pub id: String,
    pub user_name: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email: String,
    pub steam_id: Option<String>,
    pub psn_auth_code: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdateIdRequest {
    pub steam_id: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserUpdatePsnCodeRequest {
    pub psn_code: String,
    pub user_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct SignoutUserRequest {
    pub id: String,
}

impl User {
    pub fn get_user_by_id(user_id: &str) -> Result<Self, Error> {
        let conn = Database::new();

        let user = users
            .find(user_id)
            .get_result::<User>(&mut conn.pool.get().unwrap())
            .expect("Expect loading user by id");

        Ok(user)
    }

    pub fn get_user_by_username(username: &String) -> Result<Self, ApiError> {
        let conn = Database::new();

        let temp_user: User = users
            .filter(user_name.eq(username))
            .first(&mut conn.pool.get().unwrap())
            .expect("error");

        Ok(temp_user)
    }

    pub fn create_user(user: User) -> Result<Self, Error> {
        let conn = Database::new();

        let mut user = User::from(user);
        let _ = user.hash_password();

        let user = User {
            id: uuid::Uuid::new_v4().to_string(),
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            ..user
        };

        diesel::insert_into(users)
            .values(&user)
            .execute(&mut conn.pool.get().unwrap())
            .expect("Error creating new user");

        Ok(user)
    }

    pub fn update_steam_id(user_id: &str, user_steam_id: &str) -> Result<Self, ApiError> {
        let mut conn = Database::new().pool.get().unwrap();

        let mut user = users
            .find(user_id)
            .get_result::<User>(&mut conn)
            .expect("Expect loading user by id");

        user.steam_id = Some(user_steam_id.to_string());

        diesel::update(users.find(user_id))
            .set(&user)
            .execute(&mut conn)
            .expect("Failed to update user's steam_id");

        Ok(user)
    }

    pub fn update_psn_code(user_id: &str, user_psn_code: &str) -> Result<Self, ApiError> {
        let mut conn = Database::new().pool.get().unwrap();

        let mut user = users
            .find(user_id)
            .get_result::<User>(&mut conn)
            .expect("Expect loading user by id");

        user.psn_auth_code = Some(user_psn_code.to_string());

        diesel::update(users.find(user_id))
            .set(&user)
            .execute(&mut conn)
            .expect("Failed to update user's psn_aith_code");

        Ok(user)
    }

    pub fn hash_password(&mut self) -> Result<(), ApiError> {
        let salt: [u8; 32] = rand::thread_rng().gen();
        let config = Config::default();

        self.password = argon2::hash_encoded(self.password.as_bytes(), &salt, &config)
            .expect("failed to hash password");

        Ok(())
    }

    pub fn verify_password(&self, pwd: &[u8]) -> Result<bool, ApiError> {
        return argon2::verify_encoded(&self.password, pwd).map_err(|_e| panic!("wrong password"));
    }
}
