use std::fmt::{Debug, Display, Formatter};
use crate::GeneralCreationStack;
use crate::core::build::error::BuildError;
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::main::string::value::StringValue;
use crate::core::traits::error::GetBuildError;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::operation::{Operation, ProvideOperationWithNamespace};

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: String,
}

impl Operation<StringValue> for StringGetArgumentOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut GeneralCreationStack) -> Result<StringValue, BuildError> {
    let namespace = requirements.get_namespace();
    let operation = requirements.operation(namespace, &self.name);
    match operation {
      Ok(operation) => operation.build(pack_provider, requirements),
      Err(mut error) => {
        error.add_backtrace(requirements.backtrace(self));
        Err(error)
      },
    }
  }
}

impl Display for StringGetArgumentOperation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "<string:get_argument namespace=\"{}\" name=\"{}\"/>", &self.namespace, &self.name)
  }
}

impl GetBuildError for StringGetArgumentOperation {
  fn build_error(&self) -> BuildError {
    let name = self.name.to_owned();
    let namespace = self.namespace.to_owned();
    BuildError::new_operation_not_found_error(name, namespace)
  }
}