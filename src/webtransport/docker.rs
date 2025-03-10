use crate::{events::{docker::{DockerContainerInspectData, DockerContainerListData, DockerContainerRestartData, DockerEvent, DockerStatusData}, Event}, serializers::SendEvent, services::docker};

pub async fn handle_message(send_stream: &mut wtransport::SendStream, event: &DockerEvent) {
    match event {
        DockerEvent::DockerStatus { .. } => {
            send_stream.send_event(Event::Docker(DockerEvent::DockerStatus {
                data: DockerStatusData {
                    status: Some(docker::ping().await)
                }
            })).await;
            log::info!("{:?}", Event::Docker(DockerEvent::DockerContainerRestart {
                data: DockerContainerRestartData {
                    container_id: Some("container_id".to_string()),
                }
            }));
        },
        DockerEvent::DockerContainerList { .. } => {
            let containers = match docker::get_containers().await {
                Ok(containers) => containers,
                Err(error) => {
                    log::error!("Failed to get containers: {:?}", error);
                    send_stream.send_event(Event::Docker(DockerEvent::DockerStatus {
                        data: DockerStatusData {
                            status: Some(docker::ping().await)
                        }
                    })).await;
                    Vec::new()
                }
            };
            
            send_stream.send_event(Event::Docker(DockerEvent::DockerContainerList {
                data: DockerContainerListData {
                    containers: Some(containers)
                }
            })).await;
        },
        DockerEvent::DockerContainerInspect { data } => {
            match &data.container_id {
                Some(container_id) => {
                    let container = match docker::get_container(&container_id).await {
                        Ok(container) => container,
                        Err(error) => {
                            log::error!("Failed to inspect container: {:?}", error);
                            return;
                        }
                    };
                    
                    send_stream.send_event(Event::Docker(DockerEvent::DockerContainerInspect {
                        data: DockerContainerInspectData {
                            container_id: Some(container_id.clone()),
                            container: Some(container)
                        }
                    })).await;
                },
                None => {
                    log::error!("No container ID provided");
                }
            }
        },
        DockerEvent::DockerContainerStart { data } => {
            match &data.container_id {
                Some(container_id) => {
                    if let Err(error) = docker::start_container(&container_id).await {
                        log::error!("Failed to start container: {:?}", error);
                    }
                },
                None => {
                    log::error!("No container ID provided");
                }
            }
        },
        DockerEvent::DockerContainerRestart { data } => {
            match &data.container_id {
                Some(container_id) => {
                    if let Err(error) = docker::restart_container(&container_id).await {
                        log::error!("Failed to restart container: {:?}", error);
                    }
                },
                None => {
                    log::error!("No container ID provided");
                }
            }
        },
        DockerEvent::DockerContainerStop { data } => {
            match &data.container_id {
                Some(container_id) => {
                    if let Err(error) = docker::stop_container(&container_id).await {
                        log::error!("Failed to stop container: {:?}", error);
                    }
                },
                None => {
                    log::error!("No container ID provided");
                }
            }
        },
    }
}