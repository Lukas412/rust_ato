use std::str::FromStr;

use rust_decimal::Decimal;

use crate::Container;
use crate::core::data::build::{BuildError, ValueError};
use crate::core::data::number::value::NumberValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::value::Value;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl<C: Container> Buildable<NumberValue, BuildError, C> for NumberValueOperation {
  fn build(&self, requirements: &C) -> Result<NumberValue, BuildError> {
    match Decimal::from_str(&self.text) {
      Ok(value) => Ok(NumberValue::new(value, requirements.namespace().to_owned())),
      Err(error) => Err(ValueError::new(&error.to_string(), requirements.namespace())),
    }
  }
}