use std::str::FromStr;
use rust_decimal::Decimal;
use crate::Buildable;
use crate::core::build::error::BuildError;
use crate::core::main::number::value::NumberValue;
use crate::core::traits::container::Container;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "value", namespace = "number: http://www.ato.net/xmlns/number")]
pub struct NumberValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C: Container> Buildable<NumberValue, C> for NumberValueOperation {
  fn build(&self, requirements: &C) -> Result<NumberValue, BuildError> {
    let namespace = requirements.namespace().to_owned();
    match Decimal::from_str(&self.text) {
      Ok(value) => Ok(NumberValue::new(value, namespace)),
      Err(error) => Err(BuildError::new_value(error.to_string(), namespace)),
    }
  }
}