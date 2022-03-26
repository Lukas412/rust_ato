use crate::core::element::{Element, Operation};
use crate::core::element::boolean::BooleanElement;

pub struct BooleanValueOperation {
  text: String,
}

impl Operation<BooleanElement, bool> for BooleanValueOperation {
  fn build(&self) -> BooleanElement {
    let value = &self.text.to_lowercase() == "true";
    BooleanElement::new(value)
  }
}