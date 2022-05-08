use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct BooleanValue {
  value: bool,
  namespace: String,
}

impl Value for BooleanValue {
  type Type = bool;

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