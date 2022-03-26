use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;

use crate::core::element::{Element, Operation};
use crate::core::element::number::NumberElement;

pub struct NumberValueOperation {
  text: String,
}

impl Operation<NumberElement, Decimal> for NumberValueOperation {
  fn build(&self) -> NumberElement {
    match Decimal::from_str(&self.text) {
      Ok(value) => NumberElement::new(value),
      Err(_) => NumberElement::new(Decimal::zero())
    }
  }
}