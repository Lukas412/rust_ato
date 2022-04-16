mod value;

use crate::core::data::element::path::operation::value::PathValueOperation;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub enum PathOperation {
  #[yaserde(rename = "empty", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
  Empty,
  #[yaserde(rename = "value", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
  Value(PathValueOperation),
}

impl Default for PathOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl BuildableWithRequirements<StringElement, String, Requirements> for PathOperation {
  fn build_with_requirements(&self, requirements: &Requirements) -> Result<StringElement, String> {
    match self {
      Self::Empty => Ok(StringElement::new("".to_string())),
      Self::Value(operation) => operation.build_with_requirements(requirements),
    }
  }
}