use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct StringElement {
  value: String,
  namespace: String,
}

impl FromStr for StringElement {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(StringElement::new(s.to_string()))
  }
}

impl Value<String> for StringElement {
  fn new(value: String, namespace: String) -> StringElement {
    StringElement { value, namespace}
  }

  fn value(&self) -> &String {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}