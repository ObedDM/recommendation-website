use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;
use chrono::Utc;
use super::{password, errors::AuthError};
use crate::{database::connection::pg_connection, models::user::UserCredentials};

pub async fn create_user(user: UserCredentials) -> Result<String, AuthError> {
    let db: DatabaseConnection = pg_connection().await;

    let hashed_password = password::generate_password(user.password.as_str())
        .map_err(
            |e| AuthError::PasswordHashingError(e.to_string())
        )?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        username: Set(user.username),
        email: Set(user.email),
        password: Set(hashed_password),
        registration_date: Set(Utc::now().into()),
        country: Set(user.country),
    };
    
    new_user.insert(&db).await.
        map_err(
            |e| AuthError::DatabaseCreateUserError(e.to_string())
        )?;

    Ok("User created succesfully".to_string())
}






pub async fn register_user(user: Json<UserCredentials>) -> Result<(StatusCode, String), (StatusCode, String)> {

    


    Ok((
        StatusCode::OK,
        json!( {"message": "cruella"} ).to_string()
    ))

    /*
    1.- Get user information (via JSON)
    2.- Verify if the username, email dont exist in the database (return err)
    3.- call passwordEncryption::generate_password() to create a hash for the password
    4.- insert user, email, password into database (return Ok)
    */
}