use tokio::{fs::File, io::AsyncWriteExt, net::TcpListener, signal, sync::oneshot};
use dotenvy::dotenv;
use std::{path::Path, env::var};
use axum::{http::StatusCode, routing::{get, post}, serve, Router, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::cors::{CorsLayer, Any};
use tower::ServiceBuilder;
use http::{Method, header};

mod passwordEncryption;

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

    // CORS
    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    // Create routes
    let app = Router::new()
        .route("auth/signup", post(register_user))
        .route("/auth/login", post(auth_user))
        .layer(cors_layer);

    tokio::select! {
        _ = serve(listener, app) => {},
        _ = shutdown_rx => { print!("Shutting down...") }
    }
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    email: String,
    password: String
}

async fn auth_user(user: Json<UserCredentials>) -> Result<(StatusCode, String), (StatusCode, String)> {
    
    Ok((
        StatusCode::OK,
        json!( {"credentials": {
                    "user": user.username, "password": user.password
                }
        } ).to_string()
    ))
}

async fn register_user(user: Json<UserCredentials>) -> Result<(StatusCode, String), (StatusCode, String)> {
    todo!(); 

    /*
    1.- Get user information (via JSON)
    2.- Verify if the username, email dont exist in the database (return err)
    3.- call passwordEncryption::generate_password() to create a hash for the password
    4.- insert user, email, password into database (return Ok)
    */
}