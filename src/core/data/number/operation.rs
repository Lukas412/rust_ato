use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::number::value::NumberValue;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl BuildableWithRequirements<NumberValue, BuildError, ElementCreation> for NumberValueOperation {
  fn build_with_requirements(&self, _: &ElementCreation) -> Result<NumberValue, BuildError> {
    NumberValue::from_str(&self.text)
  }
}