use crate::core::data::boolean::value::BooleanValue;

use crate::core::data::build::BuildError;
use crate::core::traits::build::Buildable;
use crate::ElementContainer;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl Buildable<BooleanValue, BuildError, ElementContainer> for BooleanValueOperation {
  fn build(&self, _: &ElementContainer) -> Result<BooleanValue, BuildError> {
    BooleanValue::from_str(&self.text)
  }
}