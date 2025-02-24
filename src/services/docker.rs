use bollard::{errors::Error, secret::{ContainerInspectResponse, ContainerSummary}, system::EventsOptions, Docker};
use futures::StreamExt;
use serde_json::json;
use tokio::sync::broadcast;

pub fn get_docker_client() -> Result<Docker, Error> {
    Docker::connect_with_socket_defaults()
}

pub async fn listen_docker_events(tx: broadcast::Sender<String>) {
    let docker = match get_docker_client() {
        Ok(docker) => docker,
        Err(error) => {
            log::error!("Failed to connect to Docker: {:?}", error);
            return;
        }
    };
    
    let options = Some(EventsOptions::<String>::default());
    let mut events = docker.events(options);
    
    while let Some(event) = events.next().await {
        match event {
            Ok(event) => {
                let event_str = json!(event).to_string();
                log::info!("Docker event: [{:?}:{:?}]", event.action, event.typ);
                
                match tx.send(event_str){
                    Ok(_) => {},
                    Err(error) => {
                        log::error!("Failed to send Docker event: {:?}", error);
                    }
                };
            }
            Err(error) => {
                log::error!("Failed to receive Docker event: {:?}", error);
            }
        }
    }
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