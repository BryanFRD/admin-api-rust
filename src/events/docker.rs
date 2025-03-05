use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DockerEvent {
  DockerStatus,
  DockerContainersRestart { data: DockerContainersRestartData }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainersRestartData {
  #[serde(rename = "containerId")]
  pub container_id: String
}