use crate::core::build::error::BuildError;
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::number::operation::value::NumberValueOperation;
use crate::core::main::number::value::NumberValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::operation::Operation;
use crate::{PackProvider, Requirements};

pub mod value;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "number", namespace = "number: http://www.ato.net/xmlns/number")]
pub enum NumberOperation {
  #[yaserde(rename = "empty", prefix = "number", namespace = "number: http://www.ato.net/xmlns/number")]
  Empty,
  #[yaserde(rename = "value", prefix = "number", namespace = "number: http://www.ato.net/xmlns/number")]
  Value(NumberValueOperation),
}

impl Default for NumberOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl Operation for NumberOperation {
  type Value = NumberValue;
}

impl Buildable<NumberValue> for NumberOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &Requirements) -> Result<NumberValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(operation) => operation.build(pack_provider, requirements),
    }
  }
}