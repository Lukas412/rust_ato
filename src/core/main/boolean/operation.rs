use crate::core::build::error::BuildError;
use crate::core::main::boolean::operation::value::BooleanValueOperation;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::main::general::operation::empty::build_empty;
use crate::core::traits::operation::Operation;
use crate::{PackProvider, Requirements};

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

impl Operation<BooleanValue> for BooleanOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<BooleanValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(operation) => operation.build(pack_provider, requirements),
    }
  }
}
