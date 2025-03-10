use std::{thread::sleep, time::Duration};

use bollard::{errors::Error, secret::{ContainerInspectResponse, ContainerSummary}, system::EventsOptions, Docker};
use futures::StreamExt;
use serde_json::json;
use tokio::sync::broadcast;

use crate::{events::{docker::{DockerEvent, DockerStatusData}, Event}, serializers::SendEvent};

const INTERVAL: Duration = Duration::from_secs(10);

pub fn format_docker_event_value(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first_char) => {
            let first_char_uppercase = first_char.to_uppercase().collect::<String>();
            let rest_lowercase = chars.as_str().to_lowercase();
            format!("{}{}", first_char_uppercase, rest_lowercase)
        }
    }
}

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
                let event_action = format!("Docker{}{}", format_docker_event_value(&event.typ.clone().unwrap().to_string()), format_docker_event_value(&event.action.clone().unwrap().to_string()));
                let event_json = json!({
                    "type": event_action,
                    "data": &event.actor
                }).to_string();
                let docker_event: Event = match serde_json::from_str(&event_json) {
                    Ok(docker_event) => docker_event,
                    Err(error) => {
                        log::error!("Failed to parse Docker event [{}]: {:?}", event_action, error);
                        continue;
                    }
                };
                
                log::info!("Received Docker event: {:?}", docker_event);
                
                tx.send_event(docker_event).await;
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