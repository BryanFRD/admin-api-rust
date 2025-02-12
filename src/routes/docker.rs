use axum::{routing::{get, post}, Router};

use crate::controllers::docker;

pub fn create_routes() -> Router {
    Router::new()
        .route("/alive", get(docker::is_alive))
        .route("/containers", get(docker::get_containers))
        .route("/containers/{id}", get(docker::get_container))
        .route("/containers/{id}/start", post(docker::start_container))
        .route("/containers/{id}/stop", post(docker::stop_container))
        .route("/containers/{id}/restart", post(docker::restart_container))
}