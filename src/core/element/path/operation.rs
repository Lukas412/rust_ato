use std::path::PathBuf;
use std::str::FromStr;

use crate::core::element::path::PathElement;
use crate::core::operation::Operation;

struct PathValueOperation {
  value: String,
}

impl Operation<PathElement> for PathValueOperation {
  fn build(&self) -> PathElement {
    match PathBuf::from_str(&self.value) {
      Ok(value) => PathElement::new(value),
      Err(_) => PathElement::new(PathBuf::default())
    }
  }
}