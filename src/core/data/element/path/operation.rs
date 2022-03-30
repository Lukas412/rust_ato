use std::path::PathBuf;
use std::str::FromStr;
use crate::core::concepts::build::Buildable;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;

use crate::core::data::element::path::element::PathElement;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Buildable<PathElement> for PathValueOperation {
  fn build(&self) -> PathElement {
    match PathBuf::from_str(&self.text) {
      Ok(value) => PathElement::new(value),
      Err(_) => PathElement::new(PathBuf::default())
    }
  }
}

impl Operation<PathElement, PathBuf> for PathValueOperation {}