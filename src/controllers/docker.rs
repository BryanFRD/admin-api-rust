use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use bollard::Docker;

use crate::errors::error_status::ErrorStatus;

fn get_docker_client() -> Result<Docker, ErrorStatus> {
  Docker::connect_with_local_defaults()
    .map_err(|_| ErrorStatus::new(StatusCode::INTERNAL_SERVER_ERROR, format!("Error connecting to Docker")))
}

pub async fn is_alive() -> impl IntoResponse {
  let docker = get_docker_client();
  
  match docker {
    Ok(_) => (StatusCode::OK, Json(json!({"message": "Docker is alive"}))).into_response(),
    Err(error) => error.into_response()
  }
}

pub async fn get_containers() -> impl IntoResponse {
  let options = Some(bollard::container::ListContainersOptions::<String> {
    all: true,
    ..Default::default()
  });
  
  let docker = get_docker_client();
  
  let container = match docker {
    Ok(docker) => {
      match docker.list_containers(options).await {
        Ok(containers) => containers,
        Err(_) => return ErrorStatus::new(StatusCode::INTERNAL_SERVER_ERROR, "Error listing containers".to_string()).into_response()
      }
    },
    Err(error) => return error.into_response()
  };
  
  let response = json!({"containers": container});
  (StatusCode::OK, Json(response)).into_response()
}

pub async fn get_container(Path(id): Path<String>) -> impl IntoResponse {
  let docker = get_docker_client();
  
  let container = match docker {
    Ok(docker) => {
      match docker.inspect_container(&id, None).await {
        Ok(container) => container,
        Err(_) => return ErrorStatus::new(StatusCode::INTERNAL_SERVER_ERROR, "Error inspecting container".to_string()).into_response()
      }
    },
    Err(error) => return error.into_response()
  };
  
  let response = json!({"container": container});
  (StatusCode::OK, Json(response)).into_response()
}

pub async fn start_container(Path(id): Path<String>) -> impl IntoResponse {
  let docker = get_docker_client();
  
  match docker {
    Ok(docker) => {
      match docker.start_container(&id, None::<bollard::container::StartContainerOptions<String>>).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Container started"}))).into_response(),
        Err(err) => {
          println!("Error starting container {}", err);
          ErrorStatus::new(StatusCode::INTERNAL_SERVER_ERROR, "Error starting container".to_string())
        }.into_response()
      }
    },
    Err(error) => error.into_response()
  }
}

pub async fn stop_container(Path(id): Path<String>) -> impl IntoResponse {
  let docker = get_docker_client();
  
  match docker {
    Ok(docker) => {
      match docker.stop_container(&id, None::<bollard::container::StopContainerOptions>).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Container stopped"}))).into_response(),
        Err(_) => ErrorStatus::new(StatusCode::INTERNAL_SERVER_ERROR, "Error stopping container".to_string()).into_response()
      }
    },
    Err(error) => error.into_response()
  }
}

pub async fn restart_container(Path(id): Path<String>) -> impl IntoResponse {
  let docker = get_docker_client();
  
  match docker {
    Ok(docker) => {
      match docker.restart_container(&id, None::<bollard::container::RestartContainerOptions>).await {
        Ok(_) => (StatusCode::OK, Json(json!({"message": "Container restarted"}))).into_response(),
        Err(_) => ErrorStatus::new(StatusCode::INTERNAL_SERVER_ERROR, "Error restarting container".to_string()).into_response()
      }
    },
    Err(error) => error.into_response()
  }
}