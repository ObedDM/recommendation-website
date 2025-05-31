use axum::response::Json;
use headers::Cookie;
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use crate::services::jwt_auth::jwt_auth::auth_token;
use crate::utils::errors::JWTError;
use crate::models::jwt::Payload;

pub async fn jwt_helper(cookie_header: Cookie, db: &DatabaseConnection) -> Result<Payload, (StatusCode, Json<Value>)> {
    match auth_token(cookie_header, db).await {
        Ok(payload) => Ok(payload),

        Err(e @ JWTError::ExpiredToken) => {
            log::error!("{}", e);
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Token has expired." }))
            ))
        },

        Err(e @ JWTError::UserNotFound) => {
            log::error!("{}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "message": "User doesnt exist." }))
            ))
        },

        Err(e @ JWTError::DatabaseVerifyUserError(_)) => {
            log::error!("{}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error ocurred. Please try again later." }))
            ))
        },

        Err(e) => {
            log::error!("Unexpected error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Unexpected error ocurred. Please try again later." }))
            ))
        }
    }
}
