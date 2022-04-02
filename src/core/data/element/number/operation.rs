use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;

use crate::core::data::element::number::element::NumberElement;
use crate::core::traits::operation::Operation;
use crate::core::data::requirement::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
pub struct NumberValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<NumberElement, Requirements> for NumberValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> NumberElement {
    match Decimal::from_str(&self.text) {
      Ok(value) => NumberElement::new(value),
      Err(_) => NumberElement::new(Decimal::zero())
    }
  }
}

impl Operation<NumberElement, Decimal> for NumberValueOperation {}