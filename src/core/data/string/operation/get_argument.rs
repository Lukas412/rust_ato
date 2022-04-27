use crate::core::data::build::BuildError;
use crate::core::data::element::string::element::StringElement;
use crate::core::traits::build::BuildableWithRequirements;
use crate::ElementCreation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl BuildableWithRequirements<StringElement, BuildError, ElementCreation> for StringGetArgumentOperation {
  fn build_with_requirements(&self, requirements: &ElementCreation) -> Result<StringElement, BuildError> {
    todo!()
  }
}