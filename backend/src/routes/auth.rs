use axum::{
    extract::{Json, State},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use axum_extra::TypedHeader;
use headers::Cookie;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use log;

use crate::{models::user::{LoginCredentials, SignupCredentials}, utils::{cookies::build_cookie_header, errors::JWTError, jwt::encode_jwt}};
use crate::utils::cookies::create_jwt_cookie;
use crate::services::auth::auth;
use crate::services::auth::errors::{SignupAuthError, LoginAuthError};

/// Login handler
pub async fn login(State(db): State<DatabaseConnection>, Json(user): Json<LoginCredentials>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match auth::verify_user(&user, &db).await {
        Ok(message) => {
            log::info!("{}", message);
            // Manage Token cookies sent to browser 

            match encode_jwt(&user.username, None, &db).await {
                Ok(token) => {
                    let cookie = create_jwt_cookie(token);
                    let headers = build_cookie_header(cookie);

                    log::info!("User {} logged in successful", &user.username);
                    let body = Json(json!({ "message": "Login successful" }));

                    Ok((headers, body))
                },
        
                Err(e @ JWTError::DatabaseVerifyUserError(_)) => {
                    log::error!("{}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                    ))
                },

                Err(e @ JWTError::InvalidUserId(_)) => {
                    log::warn!("{}", e);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "Invalid Token" }))
                    ))
                },

                Err(e @ JWTError::UserIdDoestNotMatch) => {
                    log::warn!("{}", e);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "Token payload does not match" }))
                    ))
                },

                Err(e @ JWTError::UserNotFound) => {
                    log::warn!("{}", e);
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "User was not found" })),
                    ))
                },
    
                Err(e) => {
                    log::error!("Unexpected error: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                    ))
                }
            }
        }
    
        Err(e @ LoginAuthError::PasswordMatchError) => {
            log::warn!("{}", e);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Password does not match. Please try a different password." })),
            ))
        }

        Err(e @ LoginAuthError::PasswordVerificationError(_)) => {
            log::error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error occurred. Please try again later." })),
            ))
        }

        Err(e @ LoginAuthError::DatabaseVerifyUserError(_)) => {
            log::error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error occurred. Please try again later." })),
            ))
        }

        Err(e @ LoginAuthError::UserNotFound) => {
            log::warn!("{}", e);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "User was not found. Please try a different username." })),
            ))
        }
    }
}

/// Signup handler
pub async fn signup(State(db): State<DatabaseConnection>, Json(user): Json<SignupCredentials>) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match auth::create_user(user, &db).await {
        Ok(message) => {
            log::info!("{}", message);
            Ok((
                StatusCode::OK,
                Json(json!({"message": message }))
            ))
        }

        Err(e @ SignupAuthError::PasswordHashingError(_)) => {
            log::error!("{}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "Failed to sign up user. Please try again later." })),
            ))
        }

        Err(e @ SignupAuthError::DatabaseCreateUserError(_)) => {
            log::error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error occurred. Please try again later." })),
            ))
        }

        Err(e @ SignupAuthError::UsernameAlreadyExists(_)) => {
            log::warn!("{}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "Username is already in use." })),
            ))
        }

        Err(e @ SignupAuthError::EmailAlreadyExists(_)) => {
            log::warn!("{}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "Email is already in use." })),
            ))
        }

        Err(e @ SignupAuthError::UserCreationRollbackFailed(_)) => {
            log::warn!("{}", e);
            Err((
                StatusCode::OK,
                Json(json!({ "message": "User created succesfully. Failed to create default profile picture" }))
            ))
        }

        Err(e @ SignupAuthError::ProfilePictureCreationFailed(_)) => {
            log::error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error occurred. Please try again later." })),
            ))
        }
    }
}