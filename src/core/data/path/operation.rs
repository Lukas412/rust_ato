mod value;

use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::path::value::PathValue;
use crate::core::data::path::operation::value::PathValueOperation;
use crate::core::traits::build::Buildable;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub enum PathOperation {
  #[yaserde(rename = "empty", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
  Empty,
  #[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
  Value(PathValueOperation),
}

impl Default for PathOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl Buildable<PathValue, BuildError, ElementCreation> for PathOperation {
  fn build(&self, requirements: &ElementCreation) -> Result<PathValue, BuildError> {
    match self {
      Self::Empty => PathValue::from_str(""),
      Self::Value(operation) => operation.build(requirements),
    }
  }
}