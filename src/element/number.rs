use rust_decimal::Decimal;

mod operation;

struct NumberElement {
  value: Decimal,
}

impl NumberElement {
  fn new(value: Decimal) -> NumberElement {
    NumberElement { value }
  }
}