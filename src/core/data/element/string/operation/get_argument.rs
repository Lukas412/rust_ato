use crate::core::data::argument::Arguments;
use crate::core::data::build::BuildError;
use crate::core::data::element::string::element::StringElement;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl BuildableWithRequirements<StringElement, BuildError, Arguments> for StringGetArgumentOperation {
  fn build_with_requirements(&self, requirements: &Arguments) -> Result<StringElement, BuildError> {
    todo!()
  }
}