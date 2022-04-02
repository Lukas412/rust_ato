use std::path::PathBuf;
use std::str::FromStr;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::traits::operation::Operation;

use crate::core::data::element::path::element::PathElement;
use crate::core::data::requirement::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<PathElement, Requirements> for PathValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> PathElement {
    match PathBuf::from_str(&self.text) {
      Ok(value) => PathElement::new(value),
      Err(_) => PathElement::new(PathBuf::default())
    }
  }
}

impl Operation<PathElement, PathBuf> for PathValueOperation {}