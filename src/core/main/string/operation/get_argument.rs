use crate::core::build::error::BuildError;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::ProvideOperation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl<R> BuildableWithRequirements<StringValue, R> for StringGetArgumentOperation
  where R: GetNamespace + ProvideOperation<StringOperation>
{
  fn build(&self, requirements: &R) -> Result<StringValue, BuildError> {
    match requirements.operation(&self.name) {
      Some(operation) => operation.build(requirements),
      None => {
        let name = self.name.to_owned();
        let namespace = requirements.get_owned_namespace();
        Err(BuildError::new_value(name, namespace))
      },
    }
  }
}