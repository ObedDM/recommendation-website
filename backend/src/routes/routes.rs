use std::env::var;

use axum::{routing::{get, post, put, delete}, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer};
use http::{header::{AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method};
use super::{auth, home, refresh, profile};

pub fn create_router(db: DatabaseConnection) -> Router {
    let CLIENT_ADDRESS = var("CLIENT_ADDRESS").expect("Client address was not set in the environment");

    let cors_layer = CorsLayer::new()
        .allow_origin(CLIENT_ADDRESS.parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let protected_routes = Router::new()
        .route("/home", get(home::home))
        .route("/profile", get(profile::get_profile).delete(profile::delete))
        //.route("/profile/picture/post", post(profile::post_picture))
        .route("/profile/picture", put(profile::put_picture))
        //.route("/profile/picture/refresh", get(handler))
        .route("/auth/refresh", post(refresh::refresh));

    let public_routes = Router::new()
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login", post(auth::login));

    Router::new()
        .merge(protected_routes)
        .merge(public_routes)
        .layer(cors_layer)
        .with_state(db) 
}