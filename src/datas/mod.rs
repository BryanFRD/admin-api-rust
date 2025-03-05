pub mod docker;

pub trait EventDTO {
  fn to_json(&self) -> String;
}