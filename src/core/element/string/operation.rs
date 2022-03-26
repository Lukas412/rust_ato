use crate::core::element::{Element, Operation};
use crate::core::element::string::StringElement;

pub struct StringValueOperation {
  text: String,
}

impl Operation<StringElement, String> for StringValueOperation {
  fn build(&self) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}