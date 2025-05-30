use chrono::{Duration, Utc};
use entity::user;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QuerySelect, QueryFilter};
use uuid::Uuid;
use std::env::var;
use crate::models::jwt::Payload;
use super::errors::JWTError;

pub async fn encode_jwt(username: &String, user_id: Option<String>, db: &DatabaseConnection) -> Result<String, JWTError> {
    let TOKEN = var("TOKEN").expect("Token was not set in the environment");
    let secret = TOKEN;

    let now = Utc::now();
    let expire = Duration::days(2); // Expires in 2 days

    let retrieved_id: Option<(Uuid,)> = user::Entity::find()
        .select_only()
        .column(user::Column::Id)
        .filter(user::Column::Username.eq(username))
        .into_tuple::<(Uuid,)>()
        .one(db)
        .await
        .map_err(
            |e| JWTError::DatabaseVerifyUserError(e.to_string())
        )?;

    let user_id = match (user_id, retrieved_id) {
        // Checks if theres an existing id embedded from an older JWT payload, to refresh
        (Some(provided_id_str), Some((actual_id_uuid,))) => {
            let provided_id_uuid = Uuid::parse_str(&provided_id_str)
                .map_err(|e| JWTError::InvalidUserId(e.to_string()))?;

            if provided_id_uuid == actual_id_uuid {
                provided_id_uuid
            }

            else {
                return Err(JWTError::UserIdDoestNotMatch);
            }
        },
        (None, Some((actual_id,))) => actual_id,
        (_, None) => return Err(JWTError::UserNotFound),   
    };

    let payload = Payload {
        iat: now.timestamp(),
        exp: (now + expire).timestamp(),
        username: username.clone(),
        id: Some(user_id),
    };

    encode(&Header::new(Algorithm::HS256), &payload, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| JWTError::EncodingTokenError(e.to_string()))
}

pub fn decode_jwt(jwt: String) -> Result<TokenData<Payload>, JWTError> {
    let TOKEN = var("TOKEN").expect("Token was not set in the environment");
    let secret = TOKEN;

    decode(&jwt, &DecodingKey::from_secret(secret.as_ref()), &Validation::new(Algorithm::HS256))
        .map_err(|e| JWTError::DecodingTokenError(e.to_string()))
}