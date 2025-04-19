use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env::var;

pub async fn pg_connection() -> DatabaseConnection {
    dotenv().expect("Cannot access .env file");
    let DATABASE_URL = var("DATABASE_URL").expect("Database URL was not set in the environment");

    let db: DatabaseConnection = Database::connect(DATABASE_URL).await.expect("Cannot connect to database");

    return db
}