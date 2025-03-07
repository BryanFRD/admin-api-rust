use crate::events::system::SystemEvent;

pub async fn handle_message(send_stream: &mut wtransport::SendStream, event: &SystemEvent) {
    match event {
      SystemEvent::SystemStatus => {
        log::info!("SystemStatus");
      }
    }
}