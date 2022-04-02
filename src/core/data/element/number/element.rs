use rust_decimal::Decimal;

use crate::core::traits::element::Element;

#[derive(Debug)]
pub struct NumberElement {
  value: Decimal,
}

impl Element<Decimal> for NumberElement {
  fn new(value: Decimal) -> NumberElement {
    NumberElement { value }
  }
  fn value(&self) -> Decimal {
    self.value
  }
}