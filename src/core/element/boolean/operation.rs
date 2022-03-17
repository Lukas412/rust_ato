use crate::core::element::boolean::BooleanElement;
use crate::core::element::Element;
use crate::core::operation::Operation;

struct BooleanValueOperation {
  value: String,
}

impl Operation<BooleanElement> for BooleanValueOperation {
  fn build(&self) -> BooleanElement {
    let value = &self.value.to_lowercase() == "true";
    BooleanElement::new(value)
  }
}