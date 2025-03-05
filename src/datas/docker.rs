use bollard::secret::ContainerSummary;
use serde_json::json;

use super::EventDTO;

impl EventDTO for ContainerSummary {
  fn to_json(&self) -> String {
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
    }).to_string()
  }
}

impl EventDTO for Vec<ContainerSummary> {
  fn to_json(&self) -> String {
    json!(self.iter().map(|f| f.to_json()).collect::<Vec<String>>()).to_string()
  }
}