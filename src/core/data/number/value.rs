use std::str::FromStr;
use rust_decimal::Decimal;
use crate::core::data::build::{BuildError, ValueError};

use crate::core::traits::value::Value;

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

impl Value for NumberValue {
  type Type = Decimal;

  fn new(value: Self::Type, namespace: String) -> NumberValue {
    Self { value, namespace }
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}