use crate::core::build::error::BuildError;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::ProvideOperationWithNamespace;
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
    let namespace = requirements.get_namespace();
    let operation = requirements.operation(namespace, &self.name);
    match operation {
      Some(operation) => operation.build(requirements),
      None => {
        let name = self.name.to_owned();
        let namespace = namespace.to_owned();
        Err(BuildError::new_value(name, namespace))
      }
    }
  }
}