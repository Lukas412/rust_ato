use crate::core::data::element::element::Element;

#[derive(Debug)]
pub struct BooleanElement {
  value: bool,
}

impl Element<bool> for BooleanElement {
  fn new(value: bool) -> BooleanElement {
    BooleanElement { value }
  }
  fn value(&self) -> bool {
    self.value
  }
}