use std::str::FromStr;

use rust_decimal::Decimal;

use crate::core::build::error::BuildError;
use crate::core::main::number::value::NumberValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Container;
use crate::core::traits::value::Value;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl<C: Container> Buildable<NumberValue, BuildError, C> for NumberValueOperation {
  fn build(&self, requirements: &C) -> Result<NumberValue, BuildError> {
    match Decimal::from_str(&self.text) {
      Ok(value) => Ok(NumberValue::new(value, requirements.namespace().to_owned())),
      Err(error) => Err(BuildError::new_value(error.to_string(), requirements.namespace().to_owned())),
    }
  }
}