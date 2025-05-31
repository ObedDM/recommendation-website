use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::TypedHeader;
use chrono::{Duration, Utc};
use headers::Cookie;
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};

use crate::{services::jwt_auth::jwt_auth::get_token, utils::{cookies::{build_cookie_header, create_jwt_cookie}, errors::JWTError, jwt::encode_jwt, jwt_helper::jwt_helper}};

pub async fn refresh(State(db): State<DatabaseConnection>, TypedHeader(cookie_header): TypedHeader<Cookie>) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match jwt_helper(cookie_header.clone(), &db).await {
        Ok(payload) => {
            // If token expires in less than X minutes
            if (Utc::now().timestamp() - payload.exp) <= Duration::minutes(1).num_seconds() {
                let user_id = match payload.id {
                    Some(ref id) => id.to_string(),
                    None => {
                        log::error!("{}", JWTError::IdNotSetInToken);
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            Json(json!({ "message": "Token is invalid" }))
                        ))
                    }
                };

                match encode_jwt(&payload.username, Some(user_id.clone()), &db).await {
                    Ok(token) => {
                        let cookie = create_jwt_cookie(token);
                        let headers = build_cookie_header(cookie);

                        log::info!("Token for id {} refreshed successfully", &user_id);
                        let body = Json(json!({ "message": "Token refresh successful" }));
    
                        Ok((headers, body))
                    },

                    Err(e @ JWTError::DatabaseVerifyUserError(_)) => {
                        log::error!("{}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                        ))
                    },

                    Err(e @ JWTError::InvalidUserId(_)) => {
                        log::warn!("{}", e);
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            Json(json!({ "message": "Invalid Token" }))
                        ))
                    },

                    Err(e @ JWTError::UserIdDoestNotMatch) => {
                        log::warn!("{}", e);
                        return Err((
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
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(json!({ "message": "Unexpected error occurred. Please try again later." }))
                        ))
                    }
                }
            }
            
            else {
                let token = get_token(cookie_header);
                let cookie = create_jwt_cookie(token);
                let headers = build_cookie_header(cookie);

                log::info!("Token does not need refresh");
                let body = Json(json!({ "message": "Token does not need refresh" }));

                Ok((headers, body))
            }
        },

        Err((error, message)) => Err((error, message))
    }
}