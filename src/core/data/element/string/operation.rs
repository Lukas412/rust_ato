use crate::concepts::Buildable;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;
use crate::core::data::element::string::element::StringElement;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  #[yaserde(text)]
  text: String,
}

impl Buildable<StringElement> for StringValueOperation {
  fn build(&self) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}

impl Operation<StringElement, String> for StringValueOperation {}