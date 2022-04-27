use std::str::FromStr;

use crate::core::data::build::BuildError;
use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::traits::build::BuildableWithRequirements;
use crate::ElementCreation;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl BuildableWithRequirements<BooleanElement, BuildError, ElementCreation> for BooleanValueOperation {
  fn build_with_requirements(&self, _: &ElementCreation) -> Result<BooleanElement, BuildError> {
    BooleanElement::from_str(&self.text)
  }
}