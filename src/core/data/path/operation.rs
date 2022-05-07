mod value;

use crate::core::data::build::BuildError;
use crate::core::data::path::value::PathValue;
use crate::core::data::path::operation::value::PathValueOperation;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Provide;

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

impl<C: Provide<PathValue>> Buildable<PathValue, BuildError, C> for PathOperation {
  fn build(&self, requirements: &C) -> Result<PathValue, BuildError> {
    match self {
      Self::Empty => PathValue::from_str(""),
      Self::Value(operation) => operation.build(requirements),
    }
  }
}