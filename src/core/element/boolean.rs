use crate::core::element::Element;

mod operation;

pub struct BooleanElement {
  value: bool,
}

impl Element<bool> for BooleanElement {
  fn new(value: bool) -> BooleanElement {
    BooleanElement { value }
  }
  fn get_value(&self) -> bool {
    self.value
  }
}