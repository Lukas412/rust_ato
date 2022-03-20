use crate::core::element::Element;

pub mod operation;
pub mod parameter;

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