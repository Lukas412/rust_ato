use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::element::number::element::NumberElement;
use crate::core::traits::build::BuildableWithRequirements;
use crate::ElementContainer;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl BuildableWithRequirements<NumberElement, BuildError, ElementContainer> for NumberValueOperation {
  fn build_with_requirements(&self, _: &ElementContainer) -> Result<NumberElement, BuildError> {
    NumberElement::from_str(&self.text)
  }
}