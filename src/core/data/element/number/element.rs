use std::str::FromStr;
use rust_decimal::Decimal;
use crate::core::data::build::{BuildError, ValueError};

use crate::core::traits::element::Element;

#[derive(Debug)]
pub struct NumberElement {
  value: Decimal,
}

impl FromStr for NumberElement {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match Decimal::from_str(s) {
      Ok(value) => Ok(NumberElement::new(value)),
      Err(value) => Err(ValueError::new(&value.to_string()))
    }
  }
}

impl Element<Decimal> for NumberElement {
  fn new(value: Decimal) -> NumberElement {
    NumberElement { value }
  }
  fn value(&self) -> Decimal {
    self.value
  }
}