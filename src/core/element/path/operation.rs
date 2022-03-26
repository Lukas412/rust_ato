use std::path::PathBuf;
use std::str::FromStr;

use crate::core::element::{Element, Operation};
use crate::core::element::path::PathElement;

pub struct PathValueOperation {
  text: String,
}

impl Operation<PathElement, PathBuf> for PathValueOperation {
  fn build(&self) -> PathElement {
    match PathBuf::from_str(&self.text) {
      Ok(value) => PathElement::new(value),
      Err(_) => PathElement::new(PathBuf::default())
    }
  }
}