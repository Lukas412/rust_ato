use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::core::data::action::value::{Action, ActionValue};
use crate::core::data::boolean::value::BooleanValue;
use crate::core::data::number::value::NumberValue;
use crate::core::data::path::value::PathValue;
use crate::core::data::string::value::StringValue;
use crate::core::traits::value::Value;

pub enum GeneralValue {
  Action(ActionValue),
  Boolean(BooleanValue),
  Number(NumberValue),
  Path(PathValue),
  String(StringValue),
}

pub enum CombinedValue {
  Action(Action),
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Value for GeneralValue {
  type Type = CombinedValue;

  fn new(value: Self::Type, namespace: String) -> Self {
    match value {
      CombinedValue::Action(value) => Self::Action(ActionValue::new(value, namespace)),
      CombinedValue::Boolean(value) => Self::Boolean(BooleanValue::new(value, namespace)),
      CombinedValue::Number(value) => Self::Number(NumberValue::new(value, namespace)),
      CombinedValue::Path(value) => Self::Path(PathValue::new(value, namespace)),
      CombinedValue::String(value) => Self::String(StringValue::new(value, namespace)),
    }
  }

  fn value(&self) -> &Self::Type {
    match &self {
      Self::Action(value) => &CombinedValue::Action(value.value().clone()),
      Self::Boolean(value) => &CombinedValue::Boolean(*value.value()),
      Self::Number(value) => &CombinedValue::Number(*value.value()),
      Self::Path(value) => &CombinedValue::Path(value.value().to_owned()),
      Self::String(value) => &CombinedValue::String(value.value().to_owned()),
    }
  }

  fn namespace(&self) -> &String {
    match &self {
      Self::Action(value) => value.namespace(),
      Self::Boolean(value) => value.namespace(),
      Self::Number(value) => value.namespace(),
      Self::Path(value) => value.namespace(),
      Self::String(value) => value.namespace(),
    }
  }
}