use crate::core::data::build::BuildError;
use crate::core::data::string::value::StringValue;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringGetArgumentOperation {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl BuildableWithRequirements<StringValue, BuildError, ElementCreation> for StringGetArgumentOperation {
  fn build_with_requirements(&self, requirements: &ElementCreation) -> Result<StringValue, BuildError> {
    todo!()
  }
}