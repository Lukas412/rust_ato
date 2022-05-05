use std::str::FromStr;
use crate::core::data::boolean::value::BooleanValue;

use crate::core::data::build::BuildError;
use crate::core::traits::build::BuildableWithRequirements;
use crate::ElementContainer;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl BuildableWithRequirements<BooleanValue, BuildError, ElementContainer> for BooleanValueOperation {
  fn build_with_requirements(&self, _: &ElementContainer) -> Result<BooleanValue, BuildError> {
    BooleanValue::from_str(&self.text)
  }
}