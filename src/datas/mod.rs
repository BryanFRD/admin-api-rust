use serde_json::{json, Value};

use crate::events::Event;

pub mod docker;

pub fn create_event_dto(event: Event, data: Value) -> String {
  let mut event_json = serde_json::to_value(event).unwrap_or_else(|_| json!({}));
  if let Value::Object(ref mut map) = event_json {
    map.insert("data".to_string(), data);
  }
  event_json.to_string()
}

pub trait EventDTO {
  fn to_json(&self) -> Value;
}

impl EventDTO for i8 {
  fn to_json(&self) -> Value {
    json!(*self)
  }
}