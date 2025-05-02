use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, EntityTrait, ColumnTrait, QuerySelect, QueryFilter};
use uuid::Uuid;
use chrono::Utc;
use super::{password, errors::{SignupAuthError, LoginAuthError}};
use crate::models::user::{SignupCredentials, LoginCredentials};

pub async fn create_user(user: SignupCredentials, db: &DatabaseConnection) -> Result<String, SignupAuthError> {

    let hashed_password = password::generate_password(user.password.as_str())
        .map_err(
            |e| SignupAuthError::PasswordHashingError(e.to_string())
        )?;

    let new_user = user::ActiveModel {
        id: Set(Uuid::now_v7()),
        username: Set(user.username),
        email: Set(user.email.to_lowercase()),
        password: Set(hashed_password),
        registration_date: Set(Utc::now().into()),
        country: Set(user.country),
    };
    
    match new_user.insert(db).await {
        Ok(_) => Ok("User created succesfully".to_string()),
        Err(e) => {
            let err = e.to_string();
            
            if err.contains("user_email_key") {
                Err(SignupAuthError::EmailAlreadyExists(err))
            } else if err.contains("user_username_key") {
                Err(SignupAuthError::UsernameAlreadyExists(err))
            } else {
                Err(SignupAuthError::DatabaseCreateUserError(err))
            }
        }
    }
}

pub async fn verify_user(user: &LoginCredentials, db: &DatabaseConnection) -> Result<String, LoginAuthError> {

    let db_password = user::Entity::find()
        .select_only()
        .column(user::Column::Password)
        .filter(user::Column::Username.eq(&user.username))
        .into_tuple::<(String,)>()
        .one(db)
        .await
        .map_err(
            |e| LoginAuthError::DatabaseVerifyUserError(e.to_string())
        )?;

    let db_password = match db_password {
        Some((value, )) => value,
        None => return Err(LoginAuthError::UserNotFound),        
    };

    match password::check_password(user.password.as_str(), db_password.as_str()) {
        Ok(true) => Ok("Login credentials successfully validated".to_string()),
        Ok(false) => Err(LoginAuthError::PasswordMatchError),
        Err(e) => Err(LoginAuthError::PasswordVerificationError(e)),
    }
}