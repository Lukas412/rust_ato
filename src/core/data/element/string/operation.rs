use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::traits::operation::Operation;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug, YaDeserialize)]
#[yaserde(flatten)]
pub enum StringOperation {
  #[yaserde(rename = "empty", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  Empty,
  #[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
  Value(StringValueOperation),
}

impl Default for StringOperation {
  fn default() -> Self {
    Self::Empty
  }
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<StringElement, String, Requirements> for StringValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> Result<StringElement, String> {
    let value = self.text.to_owned();
    Ok(StringElement::new(value))
  }
}

impl Operation<StringElement, String> for StringValueOperation {}