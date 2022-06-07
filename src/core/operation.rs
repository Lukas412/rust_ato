use crate::{CreationStack, PackProvider};
use crate::core::error::BuildError;
use crate::core::namespace::Namespace;
use crate::core::operation::empty::build_empty;
use crate::core::operation::value::build_value;
use crate::core::value::Value;
use crate::core::variant::Variant;

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

impl Default for OperationAction {
  fn default() -> Self {
    Self::Empty
  }
}
