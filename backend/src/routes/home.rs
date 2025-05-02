use axum::{extract::State, response::Json};
use axum_extra::TypedHeader;
use headers::Cookie;
use http::StatusCode;
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};

use super::jwt_handler::jwt_helper;

pub async fn home(State(db): State<DatabaseConnection>, TypedHeader(cookie_header): TypedHeader<Cookie>) -> Result<(StatusCode, Json<Value>), (StatusCode, Json<Value>)> {
    match jwt_helper(cookie_header, &db).await {
        Ok(payload) => {
            log::info!("User token validated");
            Ok((
                StatusCode::OK,
                Json(json!({
                    "message": "User token validated.",
                    "id": payload.id,
                    "username": payload.username,
                }))
            ))
        },
        Err((error, message)) => Err((error, message))
    }
}
