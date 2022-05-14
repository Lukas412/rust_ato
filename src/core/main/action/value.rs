use crate::core::main::namespace::Namespace;
use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct ActionValue {
  value: Action,
  namespace: String,
}

impl Value for ActionValue {
  type Type = Action;

  fn default(namespace: Namespace) -> Self {
    Self { value: Action::default(), namespace }
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
pub enum Action {
  None,
  Content,
  Directory,
  Expression,
  File,
  Location,
  Output,
}

impl Default for Action {
  fn default() -> Self {
    Self::None
  }
}