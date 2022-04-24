use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::element::number::element::NumberElement;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl BuildableWithRequirements<NumberElement, BuildError, ElementArguments> for NumberValueOperation {
  fn build_with_requirements(&self, _: &ElementArguments) -> Result<NumberElement, BuildError> {
    NumberElement::from_str(&self.text)
  }
}