use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::traits::element::Element;

#[derive(Debug)]
pub struct StringElement {
  value: String,
}

impl FromStr for StringElement {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(StringElement::new(s.to_string()))
  }
}

impl Element<String> for StringElement {
  fn new(value: String) -> StringElement {
    StringElement { value }
  }
  fn value(&self) -> String {
    self.value.to_owned()
  }
}