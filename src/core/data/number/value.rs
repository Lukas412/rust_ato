use std::str::FromStr;
use rust_decimal::Decimal;
use crate::core::data::build::{BuildError, ValueError};

use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct NumberValue {
  value: Decimal,
  namespace: String,
}

impl FromStr for NumberValue {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match Decimal::from_str(s) {
      Ok(value) => Ok(NumberValue::new(value)),
      Err(value) => Err(ValueError::new(&value.to_string()))
    }
  }
}

impl Value<Decimal> for NumberValue {
  fn new(value: Decimal, namespace: String) -> NumberValue {
    NumberValue { value, namespace }
  }

  fn value(&self) -> &Decimal {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}