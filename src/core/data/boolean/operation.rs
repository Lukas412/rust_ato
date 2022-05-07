use crate::core::data::boolean::value::BooleanValue;

use crate::core::data::build::{BuildError, ValueError};
use crate::core::traits::build::Buildable;
use crate::core::traits::value::Value;
use crate::Container;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl<C: Container> Buildable<BooleanValue, BuildError, C> for BooleanValueOperation {
  fn build(&self, requirements: &C) -> Result<BooleanValue, BuildError> {
    match self.text.as_str() {
      "true" => Ok(BooleanValue::new(true, requirements.namespace().to_owned())),
      "false" => Ok(BooleanValue::new(false, requirements.namespace().to_owned())),
      value => Err(ValueError::new(value.to_owned(), requirements.namespace().to_owned()))
    }
  }
}