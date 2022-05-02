use std::str::FromStr;
use crate::core::data::build::BuildError;
use crate::core::data::string::value::StringValue;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<StringValue, BuildError, ElementCreation> for StringValueOperation {
  fn build_with_requirements(&self, _: &ElementCreation) -> Result<StringValue, BuildError> {
    StringValue::from_str(&self.text)
  }
}