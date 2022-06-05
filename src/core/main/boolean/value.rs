use crate::core::traits::namespace::Namespace;
use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct BooleanValue {
  value: bool,
  namespace: Namespace,
}

impl Value for BooleanValue {
  type Type = bool;

  fn default(namespace: Namespace) -> Self {
    Self { value: bool::default(), namespace }
  }

  fn new(value: Self::Type, namespace: Namespace) -> Self {
    Self { value, namespace }
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &Namespace {
    &self.namespace
  }
}