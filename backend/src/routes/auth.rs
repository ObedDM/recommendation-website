use axum::{http::StatusCode, Json};
use serde_json::{json, Value};
use log;

use crate::models::user::UserCredentials;
use crate::services::auth::auth;
use crate::services::auth::errors::AuthError;

pub async fn login(user: Json<UserCredentials>) -> Result<(StatusCode, Json<Value>), (StatusCode, String)> {
    
    Ok((
        StatusCode::OK,
        Json(json!(
            {"credentials": {
                "user": user.username, "email": user.email, "password": user.password
            }
        } ))
    ))
}

pub async fn signup(Json(user): Json<UserCredentials>) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {

    match auth::create_user(user).await {
        Ok(message) => {
            log::info!("{}", message);
            Ok((
                StatusCode::OK,
                Json(json!({ "message": message }))
            ))
        },

        Err(e @ AuthError::PasswordHashingError(_)) => {
            log::error!("{}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "Failed to sign up user. Please try again later" }))
            ))
        },

        Err(e @ AuthError::DatabaseCreateUserError(_)) => {
            log::error!("{}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error ocurred. Please try againt later" }))
            ))
        },

        Err(e @ AuthError::UsernameAlreadyExists(_)) => {
            log::warn!("{}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(json!( { "message": "Username is already in use" } ))
            ))
        },

        Err(e @ AuthError::EmailAlreadyExists(_)) => {
            log::warn!("{}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(json!( { "message": "Email is already in use" } ))
            ))
        },
    }
}
