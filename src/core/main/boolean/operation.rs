use crate::core::main::boolean::value::BooleanValue;

use crate::core::traits::build::Buildable;
use crate::core::traits::value::Value;
use crate::core::build::error::BuildError;
use crate::core::traits::container::Container;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl<C: Container> Buildable<BooleanValue, BuildError, C> for BooleanValueOperation {
  fn build(&self, requirements: &C) -> Result<BooleanValue, BuildError> {
    match self.text.as_str() {
      "true" => Ok(BooleanValue::new(true, requirements.namespace().to_owned())),
      "false" => Ok(BooleanValue::new(false, requirements.namespace().to_owned())),
      value => Err(BuildError::new_value(value.to_owned(), requirements.namespace().to_owned()))
    }
  }
}