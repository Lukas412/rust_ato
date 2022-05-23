use crate::core::build::error::BuildError;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::{Buildable, BuildableWithRequirements};
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::ProvideOperation;
use crate::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Buildable<StringValue> for StringGetArgumentOperation {
  fn build(&self, requirements: &Requirements) -> Result<StringValue, BuildError> {
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