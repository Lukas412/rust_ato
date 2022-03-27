use crate::core::element::{Element, Operation};
use crate::core::element::string::StringElement;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl<T> Operation<StringElement, T> for StringValueOperation {
  fn build(&self) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}