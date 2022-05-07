use crate::core::data::boolean::value::BooleanValue;

use crate::core::data::build::{BuildError, ValueError};
use crate::core::traits::build::Buildable;
use crate::core::traits::value::Value;
use crate::ElementContainer;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl Buildable<BooleanValue, BuildError, ElementContainer> for BooleanValueOperation {
  fn build(&self, _: &ElementContainer) -> Result<BooleanValue, BuildError> {
    match self.text.as_str() {
      "true" => Ok(BooleanValue::new(true, self.namespace.to_owned())),
      "false" => Ok(BooleanValue::new(false, self.namespace.to_owned())),
      value => Err(ValueError::new(value, &self.namespace))
    }
  }
}