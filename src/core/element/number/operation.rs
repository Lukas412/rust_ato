use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use crate::core::element::number::NumberElement;
use crate::core::operation::Operation;

struct NumberValueOperation {
  value: String,
}

impl Operation<NumberElement> for NumberValueOperation {
  fn build(&self) -> NumberElement {
    match Decimal::from_str(&self.value) {
      Ok(value) => NumberElement::new(value),
      Err(_) => NumberElement::new(Decimal::zero())
    }
  }
}