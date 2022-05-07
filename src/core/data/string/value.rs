use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::traits::value::Value;

#[derive(Debug)]
pub struct StringValue {
  value: String,
  namespace: String,
}

impl FromStr for StringValue {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(StringValue::new(s.to_owned()))
  }
}

impl Value for StringValue {
  type Type = String;

  fn new(value: Self::Type, namespace: String) -> Self {
    Self { value, namespace}
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}