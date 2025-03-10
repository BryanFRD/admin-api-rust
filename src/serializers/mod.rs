use serde_json::{json, Value};
use tokio::sync::broadcast;

use crate::events::Event;

pub mod docker;

pub fn create_event_dto(event: Event) -> String {
  serde_json::to_string(&event).unwrap_or_else(|_| "".to_string())
}

pub trait EventDTO {
  fn to_json(&self) -> Value;
}

impl EventDTO for i8 {
  fn to_json(&self) -> Value {
    json!(*self)
  }
}

pub trait SendEvent {
  async fn send_event(&mut self, event: Event);
}

impl SendEvent for wtransport::SendStream {
  async fn send_event(&mut self, event: Event) {
    if let Err(error) = self.write_all(create_event_dto(event).as_bytes()).await {
      log::error!("Failed to send event: {:?}", error);
    }
  }
}

impl SendEvent for broadcast::Sender<String> {
  async fn send_event(&mut self, event: Event) {
    if let Err(error) = self.send(create_event_dto(event)) {
      log::error!("Failed to send event: {:?}", error);
    }
  }
}