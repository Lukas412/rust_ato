use crate::core::data::build::BuildError;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::element::string::operation::get_argument::StringGetArgumentOperation;
use crate::core::data::element::string::operation::value::StringValueOperation;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::ElementCreation;

pub mod value;
pub mod get_argument;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub enum StringOperation {
  #[yaserde(rename = "empty", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  Empty,
  #[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  Value(StringValueOperation),
  #[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  GetArgument(StringGetArgumentOperation),
}

impl Default for StringOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl BuildableWithRequirements<StringElement, BuildError, ElementCreation> for StringOperation {
  fn build_with_requirements(&self, requirements: &ElementCreation) -> Result<StringElement, BuildError> {
    match self {
      Self::Empty => Ok(StringElement::new("".to_string())),
      Self::Value(operation) => operation.build_with_requirements(requirements),
      Self::GetArgument(operation) => operation.build_with_requirements(requirements),
    }
  }
}