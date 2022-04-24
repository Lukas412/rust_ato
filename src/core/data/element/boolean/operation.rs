use std::str::FromStr;

use crate::core::data::build::BuildError;
use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::traits::build::BuildableWithRequirements;
use crate::ElementContainer;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl BuildableWithRequirements<BooleanElement, BuildError, ElementContainer> for BooleanValueOperation {
  fn build_with_requirements(&self, _: &ElementContainer) -> Result<BooleanElement, BuildError> {
    BooleanElement::from_str(&self.text)
  }
}