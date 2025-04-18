mod server;
mod models;
mod routes;
mod services;
mod database;

#[tokio::main]
async fn main() {
    server::run_server().await; 
}
