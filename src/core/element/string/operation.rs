use crate::core::element::Element;
use crate::core::element::string::StringElement;
use crate::core::operation::Operation;

struct StringValueOperation {
  value: String,
}

impl Operation<StringElement> for StringValueOperation {
  fn build(&self) -> StringElement {
    let value = self.value.to_owned();
    StringElement::new(value)
  }
}