use std::fmt;

use axum::{body::Body, http::StatusCode, response::{IntoResponse, Response}, Json};
use serde_json::json;

#[derive(Debug)]
pub struct ErrorStatus {
  pub status: StatusCode,
  pub message: String,
}

impl ErrorStatus {
  pub fn new(status: StatusCode, message: String) -> Self {
    Self { status, message }
  }
}

impl fmt::Display for ErrorStatus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error {}: {}", self.status.as_str(), self.message)
  }
}

impl IntoResponse for ErrorStatus {
  fn into_response(self) -> Response<Body> {
      let body = json!({"error": self.message});
      (self.status, Json(body)).into_response()
  }
}