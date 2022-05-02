use std::str::FromStr;

use crate::core::data::build::BuildError;
use crate::core::data::build::ValueError;
use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct BooleanValue {
  value: bool,
  namespace: String,
}

impl FromStr for BooleanValue {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "true" => Ok(BooleanValue::new(true)),
      "false" => Ok(BooleanValue::new(false)),
      value => Err(ValueError::new(value))
    }
  }
}

impl Value<bool> for BooleanValue {
  fn new(value: bool, namespace: String) -> BooleanValue {
    BooleanValue { value, namespace }
  }

  fn value(&self) -> &bool {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}