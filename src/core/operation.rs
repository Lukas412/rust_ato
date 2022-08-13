use std::io::Read;
use std::rc::Rc;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::{Creation, CreationStack, PackProvider};
use crate::core::error::BuildError;
use crate::core::operation::action::OperationAction;
use crate::core::operation::empty::build_empty;
use crate::core::operation::get_argument::build_get_argument;
use crate::core::operation::value::build_value;
use crate::core::value::Value;
use crate::core::variant::Variant;
use crate::helpers::ser::events::start::peek_start_element;
use crate::helpers::ser::from::from_deserializer;

pub(crate) mod action;
pub(crate) mod empty;
pub(crate) mod value;
pub(crate) mod get_argument;

#[derive(Debug, Default)]
pub(crate) struct Operation {
  action: OperationAction,
  variant: Variant,
}

impl Operation {
  pub(crate) fn new(action: OperationAction, variant: Variant) -> Self {
    Self { action, variant }
  }
}

impl Operation {
  pub(crate) fn new_creation(creation: Rc<Creation>, variant: Variant) -> Self {
    let action = OperationAction::new_creation(creation);
    Operation::new(action, variant)
  }

  pub(crate) fn new_empty(variant: Variant) -> Self {
    let action = OperationAction::new_empty();
    Operation::new(action, variant)
  }

  pub(crate) fn new_value(value: String, variant: Variant) -> Self {
    let action = OperationAction::new_value(value);
    Operation::new(action, variant)
  }
}

impl Operation {
  pub(crate) fn build(&self, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    match &self.action {
      OperationAction::Empty => build_empty(&self.variant, stack),
      OperationAction::Creation(creation) => creation.clone().build(pack_provider, stack),
      OperationAction::Value(text) => build_value(&self.variant, stack, text),
      OperationAction::GetArgument(name) => build_get_argument(&self.variant, pack_provider, stack, name),
    }
  }

  pub(crate) fn is_variant(&self, variant: &Variant) -> bool {
    &self.variant == variant
  }
}

impl YaDeserialize for Operation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let (name, _, _) = peek_start_element(reader)?;
    let variant = Variant::from_owned_name(name)?;
    let action = from_deserializer(reader)?;
    Ok(Self::new(action, variant))
  }
}
