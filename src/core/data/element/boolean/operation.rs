use crate::core::concepts::build::BuildableWithRequirements;
use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;
use crate::core::data::requirement::Requirements;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
pub struct BooleanValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<BooleanElement, Requirements> for BooleanValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> BooleanElement {
    let value = self.text == "true";
    BooleanElement::new(value)
  }
}

impl Operation<BooleanElement, bool> for BooleanValueOperation {}