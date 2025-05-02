use axum::{extract::Json, http::{header, HeaderMap, HeaderValue, Response, StatusCode}, response::IntoResponse};
use cookie::{Cookie, CookieBuilder, SameSite};
use sea_orm::DatabaseConnection;
use serde_json::{json, Value};
use log;

use crate::models::user::LoginCredentials;
use crate::utils::jwt::encode_jwt;

use super::errors::JWTError;

pub async fn create_jwt_cookie(user: LoginCredentials, db: &DatabaseConnection) -> Result<Cookie, JWTError> {
    match encode_jwt(user.username, &db).await {
        Ok(token) => {

            let cookie = CookieBuilder::new("Token", token)
                .http_only(true)
                .secure(false) //<-- Set this in production with HTTPS
                .same_site(SameSite::Lax) //<-- Change this in production
                .path("/")
                .finish();
                               
                Ok(cookie)
        },

        Err(e) => Err(e)
    }
}