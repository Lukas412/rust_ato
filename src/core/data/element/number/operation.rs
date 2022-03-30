use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use crate::concepts::Buildable;

use crate::core::data::element::{Element, Operation};
use crate::core::data::element::number::element::NumberElement;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
pub struct NumberValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Buildable<NumberElement> for NumberValueOperation {
  fn build(&self) -> NumberElement {
    match Decimal::from_str(&self.text) {
      Ok(value) => NumberElement::new(value),
      Err(_) => NumberElement::new(Decimal::zero())
    }
  }
}

impl Operation<NumberElement, Decimal> for NumberValueOperation {}