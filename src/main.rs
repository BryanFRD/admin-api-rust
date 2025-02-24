mod errors;
mod webtransport;
mod services;

use rustls::crypto::{ring::default_provider, CryptoProvider};

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Info).init();
    CryptoProvider::install_default(default_provider())
        .expect("Failed to install default crypto provider");
    
    match webtransport::start_webtransport().await {
        Ok(_) => log::info!("WebTransport server stopped"),
        Err(e) => log::error!("WebTransport server failed: {:?}", e)
    };
}