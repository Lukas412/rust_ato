use crate::{PackProvider, Requirements};
use crate::core::build::error::BuildError;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::Operation;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/boolean")]
pub struct BooleanValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Operation<BooleanValue> for BooleanValueOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<BooleanValue, BuildError> {
    let namespace = requirements.get_owned_namespace();
    match self.text.as_str() {
      "true" => Ok(BooleanValue::new(true, namespace)),
      "false" => Ok(BooleanValue::new(false, namespace)),
      value => Err(BuildError::new_value(value.to_owned(), namespace))
    }
  }
}