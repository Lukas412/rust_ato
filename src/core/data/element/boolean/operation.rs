use crate::concepts::Buildable;
use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
pub struct BooleanValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Buildable<BooleanElement> for BooleanValueOperation {
  fn build(&self) -> BooleanElement {
    let value = self.text == "true";
    BooleanElement::new(value)
  }
}

impl Operation<BooleanElement, bool> for BooleanValueOperation {}