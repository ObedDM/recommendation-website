use entity::user;
use headers::Cookie;
use chrono::Utc;
use sea_orm::{DatabaseConnection, EntityTrait, ColumnTrait, QuerySelect, QueryFilter};
use uuid::Uuid;
use crate::utils::{errors::JWTError, jwt::decode_jwt};
use crate::models::jwt::Payload;

pub fn get_token(cookie_header: Cookie) -> String {
    cookie_header
        .get("Token")
        .unwrap_or("")
        .to_string()
}

pub async fn auth_token(cookie_header: Cookie, db: &DatabaseConnection) -> Result<Payload, JWTError> {
    let token = get_token(cookie_header.clone());

    println!("Full header = {:?}", cookie_header);
    println!("Token = {}", token);

    match decode_jwt(token) {
        Ok(payload) => {

            if Utc::now().timestamp() > payload.claims.exp {
                return Err(JWTError::ExpiredToken)
            }

            let user_id = user::Entity::find()
                .select_only()
                .column(user::Column::Id)
                .filter(user::Column::Id.eq(payload.claims.id))
                .into_tuple::<(Uuid,)>()
                .one(db)
                .await
                .map_err(
                    |e| JWTError::DatabaseVerifyUserError(e.to_string())
                )?;

            if user_id.is_none() {
                return Err(JWTError::UserNotFound)
            }
            
            Ok(payload.claims)
        },

        Err(e) => Err(e)   
    }
}