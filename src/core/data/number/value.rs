use std::str::FromStr;
use rust_decimal::Decimal;
use crate::core::data::build::{BuildError, ValueError};

use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct NumberElement {
  value: Decimal,
  namespace: String,
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

impl Value<Decimal> for NumberElement {
  fn new(value: Decimal, namespace: String) -> NumberElement {
    NumberElement { value, namespace }
  }

  fn value(&self) -> &Decimal {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}