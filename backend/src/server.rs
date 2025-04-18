use tokio::{net::TcpListener, signal, sync::oneshot};
use dotenvy::dotenv;
use std::env::var;
use axum::{serve, Router};

use crate::routes::create_router;

pub async fn run_server() {
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

    // Create Router
    let app: Router = create_router();

    tokio::select! {
        _ = serve(listener, app) => {},
        _ = shutdown_rx => { print!("Shutting down...") }
    }
}