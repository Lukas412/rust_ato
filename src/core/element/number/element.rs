use rust_decimal::Decimal;

use crate::core::element::Element;

pub struct NumberElement {
  value: Decimal,
}

impl Element<Decimal> for NumberElement {
  fn new(value: Decimal) -> NumberElement {
    NumberElement { value }
  }
  fn get_value(&self) -> Decimal {
    self.value
  }
}