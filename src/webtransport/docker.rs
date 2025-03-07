use crate::{datas::SendEvent, events::{docker::{DockerContainersRestartData, DockerEvent, DockerStatusData}, Event}, services::docker};

pub async fn handle_message(send_stream: &mut wtransport::SendStream, event: &DockerEvent) {
    match event {
        DockerEvent::DockerStatus { .. } => {
            send_stream.send_event(Event::Docker(DockerEvent::DockerStatus {
                data: DockerStatusData {
                    status: Some(docker::ping().await)
                }
            })).await;
            log::info!("{:?}", Event::Docker(DockerEvent::DockerContainersRestart {
                data: DockerContainersRestartData {
                    container_id: Some("container_id".to_string()),
                }
            }));
        },
        DockerEvent::DockerContainersRestart { data } => {
            log::info!("Restarting containers: {:?}", data);
        }
    }
}