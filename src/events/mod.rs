use serde::{Deserialize, Serialize};
use system::SystemEvent;
use docker::DockerEvent;

pub mod system;
pub mod docker;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Event {
  System(SystemEvent),
  Docker(DockerEvent)
}