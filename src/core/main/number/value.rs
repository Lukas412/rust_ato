use rust_decimal::Decimal;
use crate::core::traits::namespace::Namespace;

use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct NumberValue {
  value: Decimal,
  namespace: Namespace,
}

impl Value for NumberValue {
  type Type = Decimal;

  fn default(namespace: Namespace) -> Self {
    Self { value: Decimal::default(), namespace }
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