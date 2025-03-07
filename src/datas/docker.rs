use bollard::secret::ContainerSummary;
use serde_json::{json, Value};

use super::EventDTO;

impl EventDTO for ContainerSummary {
  fn to_json(&self) -> Value {
    json!({
      "id": self.id,
      "names": self.names,
      "image": self.image,
      "command": self.command,
      "created": self.created,
      "state": self.state,
      "status": self.status,
      "ports": self.ports,
      "labels": self.labels,
      "size_rw": self.size_rw,
      "size_root_fs": self.size_root_fs,
      "mounts": self.mounts,
    })
  }
}

impl EventDTO for Vec<ContainerSummary> {
  fn to_json(&self) -> Value {
    json!(self.iter().map(|f| f.to_json()).collect::<Vec<Value>>())
  }
}