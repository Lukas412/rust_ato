use crate::core::element::Element;
use crate::core::element::string::StringElement;
use crate::core::operation::Operation;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Operation<StringElement> for StringValueOperation {
  fn build(&self) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}