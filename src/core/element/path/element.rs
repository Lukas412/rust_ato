use std::path::PathBuf;

use crate::core::element::Element;

pub struct PathElement {
  value: PathBuf,
}

impl Element<PathBuf> for PathElement {
  fn new(value: PathBuf) -> PathElement {
    PathElement { value }
  }
  fn value(&self) -> PathBuf {
    self.value.to_owned()
  }
}