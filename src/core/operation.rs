use crate::{Creation, CreationStack, PackProvider};
use crate::core::error::BuildError;
use crate::core::namespace::ParameterName;
use crate::core::operation::empty::build_empty;
use crate::core::operation::get_argument::build_get_argument;
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
  pub fn new_empty(variant: Variant) -> Self {
    let action = OperationAction::Empty;
    Self::new(action, variant)
  }

  pub fn new_creation(creation: Creation, variant: Variant) -> Self {
    let action = OperationAction::Creation(creation);
    Self::new(action, variant)
  }

  pub fn new_value(text: String, variant: Variant) -> Self {
    let action = OperationAction::Value(text);
    Self::new(action, variant)
  }

  pub fn new_get_argument(name: ParameterName, variant: Variant) -> Self {
    let action = OperationAction::GetArgument(name);
    Self::new(action, variant)
  }

  fn new(action: OperationAction, variant: Variant) -> Self {
    Self { action, variant }
  }
}

impl Operation {
  fn build(&self, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    match &self.action {
      OperationAction::Empty => build_empty(&self.variant, stack),
      OperationAction::Creation(creation) => creation.build(pack_provider, stack),
      OperationAction::Value(text) => build_value(&self.variant, stack, text),
      OperationAction::GetArgument(name) => build_get_argument(&self.variant, stack, name),
    }
  }
}

#[derive(Debug)]
pub enum OperationAction {
  Empty,
  Creation(Creation),
  Value(String),
  GetArgument(ParameterName),
}

impl Default for OperationAction {
  fn default() -> Self {
    Self::Empty
  }
}
