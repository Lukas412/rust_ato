use std::path::PathBuf;
use std::str::FromStr;
use crate::core::data::build::BuildError;

use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct PathValue {
  value: PathBuf,
  namespace: String,
}

impl FromStr for PathValue {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let value = PathBuf::from(s);
    Ok(PathValue::new(value))
  }
}

impl Value<PathBuf> for PathValue {
  fn new(value: PathBuf, namespace: String) -> PathValue {
    PathValue { value, namespace }
  }

  fn value(&self) -> &PathBuf {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}