use crate::element::boolean::BooleanElement;
use crate::operation::Operation;

struct BooleanValueOperation {
  value: String,
}

impl Operation<BooleanElement> for BooleanValueOperation {
  fn build(&self) -> BooleanElement {
    let value = &self.value.to_lowercase() == "true";
    BooleanElement::new(value)
  }
}