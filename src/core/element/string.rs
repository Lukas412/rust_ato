use crate::core::element::Element;

mod operation;

struct StringElement {
  value: String,
}

impl Element<String> for StringElement {
  fn new(value: String) -> StringElement {
    StringElement { value }
  }
}