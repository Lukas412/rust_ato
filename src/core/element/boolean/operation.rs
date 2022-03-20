use crate::core::element::boolean::BooleanElement;
use crate::core::element::Element;
use crate::core::element::Operation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
pub struct BooleanValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Operation<BooleanElement> for BooleanValueOperation {
  fn build(&self) -> BooleanElement {
    let value = &self.text.to_lowercase() == "true";
    BooleanElement::new(value)
  }
}