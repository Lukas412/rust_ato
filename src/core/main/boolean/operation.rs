use crate::core::build::error::BuildError;
use crate::core::main::boolean::operation::value::BooleanValueOperation;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::main::general::operation::empty::build_empty;
use crate::core::traits::build::{Buildable, BuildableWithRequirements};
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::{Operation, ProvideOperation};
use crate::Requirements;

pub mod value;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/boolean")]
pub enum BooleanOperation {
  #[yaserde(rename = "empty", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/boolean")]
  Empty,
  #[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/boolean")]
  Value(BooleanValueOperation),
}

impl Default for BooleanOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl Operation for BooleanOperation {
  type Value = BooleanValue;
}

impl Buildable<BooleanValue> for BooleanOperation {
  fn build(&self, requirements: &Requirements) -> Result<BooleanValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(operation) => operation.build(requirements),
    }
  }
}