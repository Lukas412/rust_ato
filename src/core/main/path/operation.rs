use crate::core::build::error::BuildError;
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::path::operation::value::PathValueOperation;
use crate::core::main::path::value::PathValue;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::{Operation, ProvideOperation};

pub mod value;

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

impl Operation for PathOperation {
  type Value = PathValue;
}

impl<R> BuildableWithRequirements<PathValue, R> for PathOperation
  where R: GetNamespace + ProvideOperation<PathOperation>
{
  fn build(&self, requirements: &R) -> Result<PathValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(operation) => operation.build(requirements),
    }
  }
}