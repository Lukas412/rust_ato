use crate::core::traits::element::Element;

#[derive(Debug)]
pub struct StringElement {
  value: String,
}

impl Element<String> for StringElement {
  fn new(value: String) -> StringElement {
    StringElement { value }
  }
  fn value(&self) -> String {
    self.value.to_owned()
  }
}