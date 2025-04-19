use env_logger;
mod server;
mod models;
mod routes;
mod services;
mod database;

#[tokio::main]
async fn main() {
    env_logger::init();
    server::run_server().await; 
}
