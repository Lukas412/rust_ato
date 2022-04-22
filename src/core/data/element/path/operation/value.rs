use std::str::FromStr;
use crate::core::data::argument::Arguments;
use crate::core::data::build::BuildError;
use crate::core::data::element::path::element::PathElement;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<PathElement, BuildError, Arguments> for PathValueOperation {
  fn build_with_requirements(&self, _: &Arguments) -> Result<PathElement, BuildError> {
    PathElement::from_str(&self.text)
  }
}