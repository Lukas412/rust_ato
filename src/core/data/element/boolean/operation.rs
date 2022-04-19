use std::str::FromStr;

use crate::core::data::build::BuildError;
use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl BuildableWithRequirements<BooleanElement, BuildError, Requirements> for BooleanValueOperation {
  fn build_with_requirements(&self, _: &Requirements) -> Result<BooleanElement, BuildError> {
    BooleanElement::from_str(&self.text)
  }
}