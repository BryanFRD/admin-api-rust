use crate::{datas::{create_event_dto, EventDTO}, events::{docker::DockerEvent, Event}, services::docker};

pub async fn handle_message(send_stream: &mut wtransport::SendStream, event: &DockerEvent) {
    match event {
        DockerEvent::DockerStatus => {
            if let Err(error) = send_stream.write_all(create_event_dto(Event::Docker(DockerEvent::DockerStatus), docker::ping().await.to_json()).as_bytes()).await {
                log::error!("Failed to send event: {:?}", error);
            };
        },
        DockerEvent::DockerContainersRestart { data } => {
            log::info!("Restarting containers: {:?}", data);
        }
    }
}