use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct SignupCredentials {
    pub username: String,
    pub email: String,
    pub password: String,
    pub country: String
}

#[derive(Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String
}