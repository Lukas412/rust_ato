use crate::core::build::error::BuildError;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Container;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl<C> Buildable<StringValue, C> for StringGetArgumentOperation
  where
    C: Container + Provide<StringValue>
{
  fn build(&self, requirements: &C) -> Result<StringValue, BuildError> {
    requirements.get(&self.name, &self.namespace.as_ref().unwrap_or(&"".to_owned()))
  }
}