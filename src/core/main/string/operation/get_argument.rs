use crate::core::build::error::BuildError;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Container;
use crate::core::traits::operation::ProvideOperation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl<C> Buildable<StringValue, C> for StringGetArgumentOperation
  where C: Container + ProvideOperation<StringOperation>
{
  fn build(&self, requirements: &C) -> Result<StringValue, BuildError> {
    match requirements.operation(&self.name) {
      Some(operation) => operation.build(requirements),
      None => Err(BuildError::new_value(self.name.to_owned(), requirements.namespace().to_owned())),
    }
  }
}