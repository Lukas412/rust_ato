use std::path::PathBuf;
use std::str::FromStr;
use crate::core::data::build::BuildError;

use crate::core::traits::element::Element;

#[derive(Debug)]
pub struct PathElement {
  value: PathBuf,
}

impl FromStr for PathElement {
  type Err = BuildError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let value = PathBuf::from(s);
    Ok(PathElement::new(value))
  }
}

impl Element<PathBuf> for PathElement {
  fn new(value: PathBuf) -> PathElement {
    PathElement { value }
  }
  fn value(&self) -> PathBuf {
    self.value.to_owned()
  }
}