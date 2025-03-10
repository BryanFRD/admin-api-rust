use bollard::secret::{ContainerInspectResponse, ContainerSummary};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum DockerEvent {
  DockerStatus { data: DockerStatusData },
  DockerContainerList { data: DockerContainerListData },
  DockerContainerInspect { data: DockerContainerInspectData },
  DockerContainerStart { data: DockerContainerStartData },
  DockerContainerRestart { data: DockerContainerRestartData },
  DockerContainerStop { data: DockerContainerStopData }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerStatusData {
  pub status: Option<i8>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainerListData {
  pub containers: Option<Vec<ContainerSummary>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainerInspectData {
  #[serde(rename = "containerId", alias = "ID")]
  pub container_id: Option<String>,
  
  pub container: Option<ContainerInspectResponse>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainerStartData {
  #[serde(rename = "containerId", alias = "ID")]
  pub container_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainerRestartData {
  #[serde(rename = "containerId", alias = "ID")]
  pub container_id: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DockerContainerStopData {
  #[serde(rename = "containerId", alias = "ID")]
  pub container_id: Option<String>
}