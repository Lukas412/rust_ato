use crate::core::build::error::BuildError;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::value::Value;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C> Buildable<StringValue, C> for StringValueOperation
  where
    C: Container + Provide<StringValue>
{
  fn build(&self, requirements: &C) -> Result<StringValue, BuildError> {
    let value = self.text.to_owned();
    let namespace = requirements.namespace().to_owned();
    Ok(StringValue::new(value, namespace))
  }
}