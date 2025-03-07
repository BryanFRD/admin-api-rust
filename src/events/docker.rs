use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DockerEvent {
  DockerStatus { data: DockerStatusData },
  DockerContainersRestart { data: DockerContainersRestartData }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerStatusData {
  pub status: Option<i8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainersRestartData {
  #[serde(rename = "containerId")]
  pub container_id: Option<String>
}