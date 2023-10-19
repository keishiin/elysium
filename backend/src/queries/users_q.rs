use crate::utils::errors::ApiError;
use ::entity::{users, users::Entity as Users, users::Model as UserModel};
use axum::http::StatusCode;
use sea_orm::*;

pub async fn create_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, ApiError> {
    let user = user.insert(db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message.contains("Query Error: error returned from database: duplicate key value violates unique constraint \"users_user_name_key\"")
        {
            ApiError::new(StatusCode::BAD_REQUEST, "username has already been taken")
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, try again",
            )
        }
    })?;

    Ok(user)
}

pub async fn save_user(
    db: &DatabaseConnection,
    user: users::ActiveModel,
) -> Result<UserModel, ApiError> {
    let user = user.save(db).await.map_err(|error| {
        let error_message = error.to_string();

        if error_message.contains("Query Error: error returned from database: duplicate key value violates unique constraint \"users_user_name_key\"")
        {
            ApiError::new(StatusCode::BAD_REQUEST, "username has already been taken")
        } else {
            eprintln!("Error creating user: {:?}", error_message);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Something went wrong, try again",
            )
        }
    })?;
    convert_active_to_model(user)
}

pub async fn get_user_by_id(
    db: &DatabaseConnection,
    user_id: String,
) -> Result<UserModel, ApiError> {
    Users::find_by_id(user_id)
        .one(db)
        .await
        .map_err(|error| {
            eprint!("Error getting the user: {:?}", error);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in try again.",
            )
        })?
        .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "Incorrect username/password"))
}

pub async fn get_user_by_username(
    db: &DatabaseConnection,
    username: String,
) -> Result<UserModel, ApiError> {
    Users::find()
        .filter(users::Column::UserName.eq(username))
        .one(db)
        .await
        .map_err(|error| {
            eprint!("Error getting the user: {:?}", error);
            ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error logging in try again.",
            )
        })?
        .ok_or_else(|| ApiError::new(StatusCode::BAD_REQUEST, "Incorrect username/password"))
}

pub async fn update_steam_id_save(
    db: &DatabaseConnection,
    user: UserModel,
    steam_id: String,
) -> Result<(), ApiError> {
    let mut user = user.into_active_model();

    user.steam_id = Set(Some(steam_id));

    save_user(db, user).await?;

    Ok(())
}

pub async fn update_psn_code_save(
    db: &DatabaseConnection,
    user: UserModel,
    psn_auth_code: String,
) -> Result<(), ApiError> {
    let mut user = user.into_active_model();

    user.psn_auth_code = Set(Some(psn_auth_code));

    save_user(db, user).await?;

    Ok(())
}

fn convert_active_to_model(active_user: users::ActiveModel) -> Result<UserModel, ApiError> {
    active_user.try_into_model().map_err(|error| {
        eprintln!("Error converting task active model to model: {:?}", error);
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
