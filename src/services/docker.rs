use bollard::{errors::Error, secret::{ContainerInspectResponse, ContainerSummary}, Docker};

pub fn get_docker_client() -> Result<Docker, Error> {
    Docker::connect_with_socket_defaults()
}

pub async fn ping() -> i8 {
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.ping().await {
                Ok(_) => 1,
                Err(_) => 2
            }
        },
        Err(_) => 0
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

pub async fn get_container(id: &str) -> Result<ContainerInspectResponse, Error> {
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.inspect_container(id, None).await {
                Ok(container) => Ok(container),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}

pub async fn start_container(id: &str) -> Result<(), Error> {
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.start_container(id, None::<bollard::container::StartContainerOptions<String>>).await {
                Ok(_) => Ok(()),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}

pub async fn stop_container(id: &str) -> Result<(), Error> {
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.stop_container(id, None).await {
                Ok(_) => Ok(()),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}

pub async fn restart_container(id: &str) -> Result<(), Error> {
    let docker = get_docker_client();
    
    match docker {
        Ok(docker) => {
            match docker.restart_container(id, None).await {
                Ok(_) => Ok(()),
                Err(error) => Err(error)
            }
        },
        Err(error) => Err(error)
    }
}