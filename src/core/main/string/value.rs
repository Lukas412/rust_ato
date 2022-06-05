use crate::core::traits::namespace::Namespace;
use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct StringValue {
  value: String,
  namespace: Namespace,
}

impl Value for StringValue {
  type Type = String;

  fn default(namespace: Namespace) -> Self {
    Self { value: String::new(), namespace }
  }

  fn new(value: Self::Type, namespace: Namespace) -> Self {
    Self { value, namespace}
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &Namespace {
    &self.namespace
  }
}