use axum::Router;

mod docker;

pub fn setup_routes() -> Router {
    Router::new()
        .nest("/docker", docker::create_routes())
}