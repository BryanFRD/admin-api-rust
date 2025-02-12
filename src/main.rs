mod routes;
mod controllers;
mod errors;

use axum::http::Method;
use routes::setup_routes;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {    
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(AllowOrigin::exact("http://localhost:5173".parse().unwrap()));
    
    let app = setup_routes().layer(cors);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}