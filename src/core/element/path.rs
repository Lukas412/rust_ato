use std::path::PathBuf;
use crate::core::element::Element;

mod operation;

struct PathElement {
  value: PathBuf,
}

impl Element<PathBuf> for PathElement {
  fn new(value: PathBuf) -> PathElement {
    PathElement { value }
  }
}