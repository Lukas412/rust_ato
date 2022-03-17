use rust_decimal::Decimal;

mod operation;

pub struct NumberElement {
  value: Decimal,
}

impl NumberElement {
  pub fn new(value: Decimal) -> NumberElement {
    NumberElement { value }
  }
}