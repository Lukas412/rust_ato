use std::str::FromStr;

use crate::core::data::build::BuildError;
use crate::core::data::build::ValueError;
use crate::core::traits::element::Element;

#[derive(Debug)]
pub struct BooleanElement {
  value: bool,
}

impl FromStr for BooleanElement {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "true" => Ok(BooleanElement::new(true)),
      "false" => Ok(BooleanElement::new(false)),
      value => Err(ValueError::new(value))
    }
  }
}

impl Element<bool> for BooleanElement {
  fn new(value: bool) -> BooleanElement {
    BooleanElement { value }
  }
  fn value(&self) -> bool {
    self.value
  }
}