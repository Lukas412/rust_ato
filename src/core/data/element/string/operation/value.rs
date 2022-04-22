use std::str::FromStr;
use crate::core::data::element::argument::Arguments;
use crate::core::data::build::BuildError;
use crate::core::data::element::string::element::StringElement;
use crate::core::traits::build::BuildableWithRequirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<StringElement, BuildError, Arguments> for StringValueOperation {
  fn build_with_requirements(&self, _: &Arguments) -> Result<StringElement, BuildError> {
    StringElement::from_str(&self.text)
  }
}