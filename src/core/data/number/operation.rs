use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::number::value::NumberValue;
use crate::core::traits::build::Buildable;
use crate::ElementContainer;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl Buildable<NumberValue, BuildError, ElementContainer> for NumberValueOperation {
  fn build(&self, _: &ElementContainer) -> Result<NumberValue, BuildError> {
    NumberValue::from_str(&self.text)
  }
}