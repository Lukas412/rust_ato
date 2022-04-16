use std::path::PathBuf;
use std::str::FromStr;
use crate::core::data::element::path::element::PathElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<PathElement, String, Requirements> for PathValueOperation {
  fn build_with_requirements(&self, _: &Requirements) -> Result<PathElement, String> {
    let value = PathBuf::from(self.text.as_str());
    Ok(PathElement::new(value))
  }
}