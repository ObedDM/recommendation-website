use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserCredentials {
    pub username: String,
    pub email: String,
    pub password: String,
    pub country: String
}