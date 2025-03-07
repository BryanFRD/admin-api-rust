use std::{thread::sleep, time::Duration};

use bollard::{errors::Error, secret::{ContainerInspectResponse, ContainerSummary}, system::EventsOptions, Docker};
use futures::StreamExt;
use serde_json::json;
use tokio::sync::broadcast;

use crate::{datas::SendEvent, events::{docker::{DockerEvent, DockerStatusData}, Event}};

const INTERVAL: Duration = Duration::from_secs(10);

pub fn get_docker_client() -> Result<Docker, Error> {
    Docker::connect_with_socket_defaults()
}

pub async fn listen_docker_events(mut tx: broadcast::Sender<String>) {
    let docker = loop {
        match get_docker_client() {
            Ok(client) => break client,
            Err(error) => {
                log::error!("Failed to connect to Docker, retrying in {:?}: {:?}", INTERVAL, error);
                tx.send_event(Event::Docker(DockerEvent::DockerStatus {
                    data: DockerStatusData {
                        status: Some(0)
                    }
                })).await;
                sleep(INTERVAL);
            }
        }
    };
    
    let options = Some(EventsOptions::<String>::default());
    let mut events = docker.events(options);
    
    while let Some(event) = events.next().await {
        match event {
            Ok(event) => {
                let event_str = json!(event).to_string();
                
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