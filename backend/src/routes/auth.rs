use axum::extract::State;
use axum::{http::StatusCode, Json};
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use log;

use crate::models::user::{LoginCredentials, SignupCredentials};
use crate::services::auth::auth;
use crate::services::auth::errors::{SignupAuthError, LoginAuthError};

pub async fn login(State(db): State<DatabaseConnection>, Json(user): Json<LoginCredentials>) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    
    match auth::verify_user(user, &db).await {
        Ok(message) => {
            log::info!("{}", message);
            Ok((
                StatusCode::OK,
                Json(json!({ "message": message }))
            ))
        },

        Err(e @ LoginAuthError::PasswordMatchError) => {
            log::warn!("{}", e);
            Ok((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Password does not match. Please try a different password." }))
            ))
        },

        Err(e @ LoginAuthError::PasswordVerificationError(_)) => {
            log::error!("{}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error ocurred. Please try againt later." }))
            ))
        },

        Err(e @ LoginAuthError::DatabaseVerifyUserError(_)) => {
            log::error!("{}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error ocurred. Please try againt later." }))
            ))
        },

        Err(e @ LoginAuthError::UserNotFound) => {
            log::warn!("{}", e);
            Ok((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "User was not found. Please try a different username" }))
            ))
        },
    }
}

pub async fn signup(State(db): State<DatabaseConnection>, Json(user): Json<SignupCredentials>,) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {

    match auth::create_user(user, &db).await {
        Ok(message) => {
            log::info!("{}", message);
            Ok((
                StatusCode::OK,
                Json(json!({ "message": message }))
            ))
        },

        Err(e @ SignupAuthError::PasswordHashingError(_)) => {
            log::error!("{}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "Failed to sign up user. Please try again later." }))
            ))
        },

        Err(e @ SignupAuthError::DatabaseCreateUserError(_)) => {
            log::error!("{}", e);
            Ok((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error ocurred. Please try againt later." }))
            ))
        },

        Err(e @ SignupAuthError::UsernameAlreadyExists(_)) => {
            log::warn!("{}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(json!( { "message": "Username is already in use." } ))
            ))
        },

        Err(e @ SignupAuthError::EmailAlreadyExists(_)) => {
            log::warn!("{}", e);
            Ok((
                StatusCode::BAD_REQUEST,
                Json(json!( { "message": "Email is already in use." } ))
            ))
        },
    }
}
