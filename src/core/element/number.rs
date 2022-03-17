use rust_decimal::Decimal;
use crate::core::element::Element;

mod operation;

pub struct NumberElement {
  value: Decimal,
}

impl Element<Decimal> for NumberElement {
  fn new(value: Decimal) -> NumberElement {
    NumberElement { value }
  }
}