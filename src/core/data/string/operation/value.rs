use crate::core::data::build::BuildError;
use crate::core::data::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Provide;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<C: Provide<StringValue>> Buildable<StringValue, BuildError, C> for StringValueOperation {
  fn build(&self, _: &C) -> Result<StringValue, BuildError> {
    StringValue::from_str(&self.text)
  }
}