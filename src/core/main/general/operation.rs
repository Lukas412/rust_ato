use crate::{BuildError, CreationStack, GeneralCreation};
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::main::general::variant::Variant;
use crate::core::main::string::value::StringValue;
use crate::core::traits::namespace::Namespace;

pub mod empty;

#[derive(Debug)]
pub struct Operation {
  action: OperationAction,
  variant: Variant,
}

impl Operation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut CreationStack) -> Result<StringValue, BuildError> {
    match &self.action {
      OperationAction::Empty => build_empty(requirements),
      OperationAction::Value { text } => todo!(),
      OperationAction::GetArgument { name, namespace } => todo!(),
    }
  }
}

#[derive(Debug, YaDeserialize)]
pub enum OperationAction {
  #[yaserde(rename = "empty")]
  Empty,
  #[yaserde(rename = "value")]
  Value {
    #[yaserde(text)]
    text: String,
  },
  #[yaserde(rename = "get_argument")]
  GetArgument {
    #[yaserde(attribute)]
    name: String,
    #[yaserde(attribute)]
    namespace: Namespace,
  },
}
