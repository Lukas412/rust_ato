mod value;

use std::path::PathBuf;
use crate::core::build::error::BuildError;
use crate::core::main::path::value::PathValue;
use crate::core::main::path::operation::value::PathValueOperation;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Container;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
pub enum PathOperation {
  #[yaserde(rename = "empty", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
  Empty,
  #[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
  Value(PathValueOperation),
}

impl Default for PathOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl<C> Buildable<PathValue, C> for PathOperation
  where
    C: Container + Provide<PathValue>
{
  fn build(&self, requirements: &C) -> Result<PathValue, BuildError> {
    match self {
      Self::Empty => Ok(PathValue::new(PathBuf::default(), requirements.namespace().to_owned())),
      Self::Value(operation) => operation.build(requirements),
    }
  }
}