use axum::{Router, routing::get};

use crate::controllers::docker;

pub fn create_routes() -> Router {
    Router::new()
        .route("/", get(docker::get_containers))
        .route("/{id}", get(docker::get_container))
        .route("/start/{id}", get(docker::start_container))
        .route("/stop/{id}", get(docker::stop_container))
        .route("/restart/{id}", get(docker::restart_container))
}