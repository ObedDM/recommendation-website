use axum::{routing::{get, post}, Router};
use tower_http::cors::{CorsLayer, Any};
use http::{Method, header};

pub mod auth;

pub fn create_router() -> Router {

    let cors_layer = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE]);

    Router::new()
        .route("/auth/signup", post(auth::register_user))
        .route("/auth/login", post(auth::auth_user))
        .layer(cors_layer)
}