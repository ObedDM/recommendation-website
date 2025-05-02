use env_logger;
use sea_orm::DatabaseConnection;
use dotenvy::dotenv;
use crate::database::connection::pg_connection;
mod server;
mod models;
mod routes;
mod services;
mod database;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    let db: DatabaseConnection = pg_connection().await;

    server::run_server(db).await; 
}
