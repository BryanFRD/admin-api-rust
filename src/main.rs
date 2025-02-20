mod routes;
mod controllers;
mod errors;
mod webtransport;
mod services;

use axum::http::{HeaderValue, Method};
use routes::setup_routes;
use rustls::crypto::{ring::default_provider, CryptoProvider};
use tower_http::cors::{AllowOrigin, CorsLayer};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    CryptoProvider::install_default(default_provider())
        .expect("Failed to install default crypto provider");
    
    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(AllowOrigin::list([
            HeaderValue::from_str("https://admin.bryan-ferrando.fr").unwrap(),
            HeaderValue::from_str("http://localhost:5173").unwrap()
        ]));
    
    let app = setup_routes()
        .layer(cors);
    
    tokio::spawn(async move {
        match webtransport::start_webtransport().await {
            Ok(_) => log::info!("WebTransport server stopped"),
            Err(e) => log::error!("WebTransport server failed: {:?}", e)
        };
    });
    
    tokio::spawn(async move {
        match webtransport::start_client().await {
            Ok(_) => log::info!("WebTransport client stopped"),
            Err(e) => log::error!("WebTransport client failed: {:?}", e)
        };
    });
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    log::info!("Listening on {}", listener.local_addr().unwrap());
    
    axum::serve(listener, app).await.unwrap();
}