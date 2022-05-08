use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::core::data::action::value::Action;
use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct  GeneralValue {
  value: CombinedGeneralValue,
  namespace: String,
}

#[derive(Debug, Clone)]
pub enum CombinedGeneralValue {
  Action(Action),
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Value for GeneralValue {
  type Type = CombinedGeneralValue;

  fn new(value: Self::Type, namespace: String) -> Self {
    Self { value, namespace }
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}