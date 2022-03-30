use crate::core::concepts::build::BuildableWithRequirements;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::requirement::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<StringElement, Requirements> for StringValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}

impl Operation<StringElement, String> for StringValueOperation {}