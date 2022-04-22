use std::str::FromStr;
use crate::core::data::element::argument::Arguments;

use crate::core::data::build::BuildError;
use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl BuildableWithRequirements<BooleanElement, BuildError, Arguments> for BooleanValueOperation {
  fn build_with_requirements(&self, _: &Arguments) -> Result<BooleanElement, BuildError> {
    BooleanElement::from_str(&self.text)
  }
}