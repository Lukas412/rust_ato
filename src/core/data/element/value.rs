use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct ElementValue {
  value: CombinedElementValue,
  namespace: String,
}

#[derive(Debug, Clone)]
pub enum CombinedElementValue {
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Value for ElementValue {
  type Type = CombinedElementValue;

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