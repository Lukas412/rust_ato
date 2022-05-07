use rust_decimal::Decimal;

use crate::core::traits::value::Value;

#[derive(Debug)]
pub struct NumberValue {
  value: Decimal,
  namespace: String,
}

impl Value for NumberValue {
  type Type = Decimal;

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