use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Payload {
    pub exp: i64,
    pub iat: i64,
    pub username: String,
    pub id: Uuid,
}