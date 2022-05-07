use std::path::PathBuf;
use std::str::FromStr;
use crate::core::data::build::BuildError;

use crate::core::traits::value::Value;

#[derive(Debug)]
pub struct PathValue {
  value: PathBuf,
  namespace: String,
}

impl Value for PathValue {
  type Type = PathBuf;

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