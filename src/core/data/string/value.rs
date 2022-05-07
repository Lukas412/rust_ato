use crate::core::traits::value::Value;

#[derive(Debug)]
pub struct StringValue {
  value: String,
  namespace: String,
}

impl Value for StringValue {
  type Type = String;

  fn new(value: Self::Type, namespace: String) -> Self {
    Self { value, namespace}
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}