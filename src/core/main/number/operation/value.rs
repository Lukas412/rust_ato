use std::str::FromStr;
use rust_decimal::Decimal;
use crate::BuildableWithRequirements;
use crate::core::build::error::BuildError;
use crate::core::main::number::value::NumberValue;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "value", namespace = "number: http://www.ato.net/xmlns/number")]
pub struct NumberValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<R> BuildableWithRequirements<NumberValue, R> for NumberValueOperation
  where R: GetNamespace
{
  fn build(&self, requirements: &R) -> Result<NumberValue, BuildError> {
    let namespace = requirements.get_owned_namespace();
    match Decimal::from_str(&self.text) {
      Ok(value) => Ok(NumberValue::new(value, namespace)),
      Err(error) => Err(BuildError::new_value(error.to_string(), namespace)),
    }
  }
}