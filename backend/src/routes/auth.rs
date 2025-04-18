use axum::{Json, http::StatusCode};
use serde_json::json;
use crate::models::user::UserCredentials;

pub async fn auth_user(user: Json<UserCredentials>) -> Result<(StatusCode, String), (StatusCode, String)> {
    
    Ok((
        StatusCode::OK,
        json!( {"credentials": {
                    "user": user.username, "email": user.email, "password": user.password
                }
        } ).to_string()
    ))
}

pub async fn register_user(user: Json<UserCredentials>) -> Result<(StatusCode, String), (StatusCode, String)> {

    Ok((
        StatusCode::OK,
        json!( {"message": "cruella"} ).to_string()
    ))

    /*
    1.- Get user information (via JSON)
    2.- Verify if the username, email dont exist in the database (return err)
    3.- call passwordEncryption::generate_password() to create a hash for the password
    4.- insert user, email, password into database (return Ok)
    */
}