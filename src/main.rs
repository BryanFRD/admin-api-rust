mod routes;
mod controllers;
mod errors;

use routes::setup_routes;

#[tokio::main]
async fn main() {
    let app = setup_routes();
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}