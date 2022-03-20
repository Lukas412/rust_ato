use std::path::PathBuf;
use std::str::FromStr;
use crate::core::element::Element;

use crate::core::element::path::PathElement;
use crate::core::operation::Operation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Operation<PathElement> for PathValueOperation {
  fn build(&self) -> PathElement {
    match PathBuf::from_str(&self.text) {
      Ok(value) => PathElement::new(value),
      Err(_) => PathElement::new(PathBuf::default())
    }
  }
}