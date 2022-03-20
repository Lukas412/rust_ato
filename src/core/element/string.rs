use crate::core::element::Element;

pub mod operation;

struct StringElement {
  value: String,
}

impl Element<String> for StringElement {
  fn new(value: String) -> StringElement {
    StringElement { value }
  }
  fn get_value(&self) -> String {
    self.value.to_owned()
  }
}