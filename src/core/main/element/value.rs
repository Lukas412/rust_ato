use std::path::PathBuf;

use rust_decimal::Decimal;
use crate::core::main::namespace::Namespace;

use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct ElementValue {
  value: CombinedElementValue,
  namespace: String,
}

#[derive(Debug, Clone)]
pub enum CombinedElementValue {
  None,
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Default for CombinedElementValue {
  const fn default() -> Self {
    Self::None
  }
}

impl Value for ElementValue {
  type Type = CombinedElementValue;

  fn default(namespace: Namespace) -> Self {
    Self { value: CombinedElementValue::default(), namespace }
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