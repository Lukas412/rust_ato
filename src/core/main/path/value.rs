use std::path::PathBuf;
use crate::core::main::namespace::Namespace;

use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub struct PathValue {
  value: PathBuf,
  namespace: String,
}

impl Value for PathValue {
  type Type = PathBuf;

  fn default(namespace: Namespace) -> Self {
    PathValue { value: PathBuf::default(), namespace }
  }

  fn new(value: Self::Type, namespace: String) -> Self {
    Self {
      value,
      namespace
    }
  }

  fn value(&self) -> &Self::Type {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}