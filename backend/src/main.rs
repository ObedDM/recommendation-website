use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener, signal, sync::oneshot};
use dotenvy::dotenv;
use std::{path::Path, env::var};
use axum::{http::StatusCode, routing::post, serve, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[tokio::main]
async fn main() {
    // Open .env
    dotenv().expect("Cannot access .env file");
    let SERVER_ADDRESS = var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_string());

    // Shutdown setup [Linux]
    /*let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        let mut sigint = signal(SignalKind::interrupt()).unwrap();
        sigint.recv().await;
        let _ = shutdown_tx.send(());
    });*/

    // Shutdown setup [Windows]
    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    tokio::spawn(async move {
        signal::ctrl_c().await.expect("failed to listen for ctrl_c");
        let _ = shutdown_tx.send(());
    });

    // TcpListener
    let listener = TcpListener::bind(SERVER_ADDRESS).await.expect("Could not create TcpListener");
    println!("Listening on: {}", listener.local_addr().unwrap());

    // Create routes
    let app = Router::new()
            .route("/auth/login", post(post_credentials));

    tokio::select! {
        _ = serve(listener, app) => {},
        _ = shutdown_rx => { print!("Shutting down...") }
    }
    
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

async fn post_credentials(user: Json<UserCredentials>) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!();
}