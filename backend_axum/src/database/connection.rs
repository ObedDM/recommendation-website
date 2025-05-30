use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{env::var, time::Duration};

pub async fn pg_connection() -> DatabaseConnection {
    let DATABASE_URL = var("DATABASE_URL").expect("Database URL was not set in the environment");

    let mut opt = ConnectOptions::new(DATABASE_URL);
    opt.max_connections(16)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .sqlx_logging(true);
    
    Database::connect(opt).await.expect("Cannot connect to database")
}