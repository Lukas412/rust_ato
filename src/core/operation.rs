use std::io::Read;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use core::operation::action::OperationAction;
use core::parse::{from_deserializer, peek_start_element};
use Creation;

use crate::{CreationStack, PackProvider};
use crate::core::error::BuildError;
use crate::core::operation::empty::build_empty;
use crate::core::operation::get_argument::build_get_argument;
use crate::core::operation::value::build_value;
use crate::core::value::Value;
use crate::core::variant::Variant;

pub mod action;
pub mod empty;
pub mod value;
pub mod get_argument;

#[derive(Debug, Default)]
pub struct Operation {
  action: OperationAction,
  variant: Variant,
}

impl Operation {
  pub fn new(action: OperationAction, variant: Variant) -> Self {
    Self { action, variant }
  }
}

impl Operation {
  pub fn new_creation(creation: Creation, variant: Variant) -> Self {
    let action = OperationAction::new_creation(creation);
    Operation::new(action, variant)
  }

  pub fn new_empty(variant: Variant) -> Self {
    let action = OperationAction::new_empty();
    Operation::new(action, variant)
  }

  pub fn new_value(value: String, variant: Variant) -> Self {
    let action = OperationAction::new_value(value);
    Operation::new(action, variant)
  }
}

impl Operation {
  pub fn build(&self, pack_provider: &PackProvider, stack: &mut CreationStack) -> Result<Value, BuildError> {
    match &self.action {
      OperationAction::Empty => build_empty(&self.variant, stack),
      OperationAction::Creation(creation) => creation.build(pack_provider, stack),
      OperationAction::Value(text) => build_value(&self.variant, stack, text),
      OperationAction::GetArgument(name) => build_get_argument(&self.variant, pack_provider, stack, name),
    }
  }

  pub fn is_variant(&self, variant: &Variant) -> bool {
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
