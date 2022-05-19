use crate::BuildableWithRequirements;
use crate::core::build::error::BuildError;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/boolean")]
pub struct BooleanValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<R> BuildableWithRequirements<BooleanValue, R> for BooleanValueOperation
  where R: GetNamespace
{
  fn build(&self, requirements: &R) -> Result<BooleanValue, BuildError> {
    let namespace = requirements.get_owned_namespace();
    match self.text.as_str() {
      "true" => Ok(BooleanValue::new(true, namespace)),
      "false" => Ok(BooleanValue::new(false, namespace)),
      value => Err(BuildError::new_value(value.to_owned(), namespace))
    }
  }
}