use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::core::main::action::value::Action;
use crate::core::main::namespace::Namespace;
use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct  GeneralValue {
  value: CombinedGeneralValue,
  namespace: String,
}

impl Value for GeneralValue {
  type Type = CombinedGeneralValue;

  fn default(namespace: Namespace) -> Self {
    Self { value: CombinedGeneralValue::default(), namespace }
  }

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

#[derive(Debug, Clone)]
pub enum CombinedGeneralValue {
  None,
  Action(Action),
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Default for CombinedGeneralValue {
  fn default() -> Self {
    Self::None
  }
}
