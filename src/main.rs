mod routes;
mod controllers;
mod errors;
mod quic;
mod services;

use axum::http::{HeaderValue, Method};
use routes::setup_routes;
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();
    
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(AllowOrigin::list([
            HeaderValue::from_str("https://admin.bryan-ferrando.fr").unwrap(),
            HeaderValue::from_str("http://localhost:5173").unwrap()
        ]));
    
    let app = setup_routes()
        .layer(cors);
    
    tokio::spawn(async {
        quic::start_webtransport().await.unwrap();
    });
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}