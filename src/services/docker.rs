use serde_json::json;
use bollard::{container::ListContainersOptions, errors::Error, secret::ContainerSummary, system::Version, Docker};

pub fn get_docker_client() -> Result<Docker, Error> {
    Docker::connect_with_socket_defaults()
}

pub async fn get_version() -> Result<Version, Error> {
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.version().await {
                Ok(version) => Ok(version),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}

pub async fn get_containers() -> Result<Vec<ContainerSummary>, Error> {
    let options = Some(bollard::container::ListContainersOptions::<String> {
        all: true,
        ..Default::default()
    });
    
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.list_containers(options).await {
                Ok(containers) => Ok(containers),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}