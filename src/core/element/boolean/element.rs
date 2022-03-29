use crate::core::element::Element;

pub struct BooleanElement {
  value: bool,
}

impl Elesment<bool> for BooleanElement {
  fn new(value: bool) -> BooleanElement {
    BooleanElement { value }
  }
  fn get_value(&self) -> bool {
    self.value
  }
}