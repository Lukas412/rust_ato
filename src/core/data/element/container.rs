use crate::core::data::element::Element;
use crate::core::traits::container::Container;

pub struct ElementContainer {
  elements: Vec<Element>,
}

impl Container<Element> for ElementContainer {
  fn new(values: Vec<Element>) -> Self {
    Self { elements: values }
  }
  fn value(&self, namespace: String, name: String) -> Element {
    todo!()
  }
}