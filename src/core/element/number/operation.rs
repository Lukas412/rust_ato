use std::str::FromStr;
use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use crate::core::element::Element;
use crate::core::element::number::NumberElement;
use crate::core::operation::Operation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
struct NumberValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Operation<NumberElement> for NumberValueOperation {
  fn build(&self) -> NumberElement {
    match Decimal::from_str(&self.text) {
      Ok(value) => NumberElement::new(value),
      Err(_) => NumberElement::new(Decimal::zero())
    }
  }
}