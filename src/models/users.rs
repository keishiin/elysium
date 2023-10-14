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
pub struct UserSignOutRequest {
    pub user_id: String,
    pub password: String,
}
// impl User {
//     pub async fn get_user_by_id(db: &DbConn, user_id: &str) -> Result<Option<users::Model>, DbErr> {
//         return Users::find_by_id(user_id)
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find user.".to_owned()))
//             .map(Into::into);
//     }

//     pub async fn get_user_by_username(
//         db: &DbConn,
//         username: &String,
//     ) -> Result<Option<users::Model>, DbErr> {
//         return Users::find()
//             .filter(users::Column::UserName.contains(username))
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
//             .map(Into::into);
//     }

//     pub async fn update_steam_id(
//         db: &DbConn,
//         user_id: &str,
//         user_steam_id: &str,
//     ) -> Result<users::Model, DbErr> {
//         let user: users::ActiveModel = Users::find_by_id(user_id)
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
//             .map(Into::into)?;

//         users::ActiveModel {
//             id: user.id,
//             user_name: user.user_name,
//             password: user.password,
//             email: user.email,
//             steam_id: Set(Some(user_steam_id.to_owned())),
//             ..Default::default()
//         }
//         .update(db)
//         .await
//     }

//     pub async fn update_psn_code(
//         db: &DbConn,
//         user_id: &str,
//         user_psn_code: &str,
//     ) -> Result<users::Model, DbErr> {
//         let user: users::ActiveModel = Users::find_by_id(user_id)
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
//             .map(Into::into)?;

//         users::ActiveModel {
//             id: user.id,
//             user_name: user.user_name,
//             password: user.password,
//             email: user.email,
//             psn_auth_code: Set(Some(user_psn_code.to_owned())),
//             ..Default::default()
//         }
//         .update(db)
//         .await
//     }

//     pub async fn delete_user_by_id(db: &DbConn, user_id: String) -> Result<DeleteResult, DbErr> {
//         let user: users::ActiveModel = Users::find_by_id(user_id)
//             .one(db)
//             .await?
//             .ok_or(DbErr::Custom("Cannot find post.".to_owned()))
//             .map(Into::into)?;

//         user.delete(db).await
//     }
// }
