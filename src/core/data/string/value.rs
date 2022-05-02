use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct StringValue {
  value: String,
  namespace: String,
}

impl FromStr for StringValue {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(StringValue::new(s.to_string()))
  }
}

impl Value<String> for StringValue {
  fn new(value: String, namespace: String) -> StringValue {
    StringValue { value, namespace}
  }

  fn value(&self) -> &String {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}