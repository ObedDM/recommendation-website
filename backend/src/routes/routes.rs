use axum::{routing::{get, post}, Router};
use sea_orm::DatabaseConnection;
use tower_http::cors::{CorsLayer, Any};
use http::{Method, header};
use super::auth;

pub fn create_router(db: DatabaseConnection) -> Router {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    Router::new()
        .route("/auth/signup", post(auth::signup))
        .route("/auth/login", post(auth::login))
        .layer(cors_layer)
        .with_state(db)
}