use entity::user;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, EntityTrait, ColumnTrait, QuerySelect, QueryFilter};
use uuid::Uuid;
use chrono::Utc;
use super::{errors::{LoginAuthError, SignupAuthError}, password};
use crate::{models::user::{LoginCredentials, SignupCredentials}, services::profile::profile::create_default_profile_picture};

pub async fn create_user(user: SignupCredentials, db: &DatabaseConnection) -> Result<String, SignupAuthError> {

    let hashed_password = password::generate_password(user.password.as_str())
        .map_err(
            |e| SignupAuthError::PasswordHashingError(e.to_string())
        )?;

    let new_user_id: Uuid = Uuid::now_v7();

    let new_user = user::ActiveModel {
        id: Set(new_user_id),
        username: Set(user.username),
        email: Set(user.email.to_lowercase()),
        password: Set(hashed_password),
        registration_date: Set(Utc::now().into()),
        country: Set(user.country),
    };

    new_user.insert(db).await
    .map_err(|e| {
        let err = e.to_string();

        if err.contains("user_username_key") {
            SignupAuthError::UsernameAlreadyExists(err)
        } else if err.contains("user_email_key") {
            SignupAuthError::EmailAlreadyExists(err)
        } else {
            SignupAuthError::DatabaseCreateUserError(err)
        }
    })?;
    
    if let Err(picture_err) = create_default_profile_picture(new_user_id).await {
        if let Err(delete_err) = user::Entity::delete_by_id(new_user_id).exec(db).await {
            
            return Err(SignupAuthError::UserCreationRollbackFailed(delete_err.to_string()))
        }

        return Err(SignupAuthError::ProfilePictureCreationFailed(picture_err.to_string()))
    }

    Ok("User created successfully".to_string())
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