use crate::{BuildError, CreationStack, Creation};
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::general::operation::value::build_value;
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::main::general::value::Value;
use crate::core::main::general::variant::Variant;
use crate::core::main::string::value::StringValue;
use crate::core::main::namespace::Namespace;

pub mod empty;
pub mod value;
pub mod get_argument;

#[derive(Debug)]
pub struct Operation {
  action: OperationAction,
  variant: Variant,
}

impl Operation {
  fn build(&self, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    match &self.action {
      OperationAction::Empty => build_empty(&self.variant, stack),
      OperationAction::Value { text } => build_value(&self.variant, stack, text),
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
