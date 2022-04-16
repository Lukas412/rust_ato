mod value;

use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::element::path::element::PathElement;
use crate::core::data::element::path::operation::value::PathValueOperation;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;

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

impl BuildableWithRequirements<PathElement, BuildError, Requirements> for PathOperation {
  fn build_with_requirements(&self, requirements: &Requirements) -> Result<PathElement, BuildError> {
    match self {
      Self::Empty => PathElement::from_str(""),
      Self::Value(operation) => operation.build_with_requirements(requirements),
    }
  }
}