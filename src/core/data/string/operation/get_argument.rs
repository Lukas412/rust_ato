use crate::core::data::build::BuildError;
use crate::core::data::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Provide;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl<C: Provide<StringValue>> Buildable<StringValue, BuildError, C> for StringGetArgumentOperation {
  fn build(&self, requirements: &C) -> Result<StringValue, BuildError> {
    todo!()
  }
}