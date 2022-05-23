use crate::core::build::error::BuildError;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::value::Value;
use crate::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl StringValueOperation {
  pub fn new(text: String) -> StringOperation {
    let inner = Self { text };
    StringOperation::Value(inner)
  }
}

impl Buildable<StringValue> for StringValueOperation {
  fn build(&self, requirements: &Requirements) -> Result<StringValue, BuildError> {
    let value = self.text.to_owned();
    let namespace = requirements.get_owned_namespace();
    Ok(StringValue::new(value, namespace))
  }
}