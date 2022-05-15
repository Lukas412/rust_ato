use crate::Buildable;
use crate::core::build::error::BuildError;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::traits::container::Container;
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/boolean")]
pub struct BooleanValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C> Buildable<BooleanValue, C> for BooleanValueOperation
  where C: Container
{
  fn build(&self, requirements: &C) -> Result<BooleanValue, BuildError> {
    let namespace = requirements.namespace().to_owned();
    match self.text.as_str() {
      "true" => Ok(BooleanValue::new(true, namespace)),
      "false" => Ok(BooleanValue::new(false, namespace)),
      value => Err(BuildError::new_value(value.to_owned(), namespace))
    }
  }
}