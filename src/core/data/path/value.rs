use std::path::PathBuf;
use std::str::FromStr;
use crate::core::data::build::BuildError;

use crate::core::traits::element::Value;

#[derive(Debug)]
pub struct PathElement {
  value: PathBuf,
  namespace: String,
}

impl FromStr for PathElement {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let value = PathBuf::from(s);
    Ok(PathElement::new(value))
  }
}

impl Value<PathBuf> for PathElement {
  fn new(value: PathBuf, namespace: String) -> PathElement {
    PathElement { value, namespace }
  }

  fn value(&self) -> &PathBuf {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }
}