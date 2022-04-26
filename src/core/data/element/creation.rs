use crate::core::data::element::Element;
use crate::core::traits::creation::{CreationValue, Creation};

pub struct ElementCreation {
  elements: Vec<ElementCreationValue>,
}

impl Creation<ElementCreationValue, Element> for ElementCreation {
  fn new(values: Vec<ElementCreationValue>) -> Self {
    Self { elements: values }
  }

  fn value(&self, namespace: String, name: String) -> &ElementCreationValue {
    todo!()
  }
}

pub struct ElementCreationValue {
  value: Element,
  namespace: String,
  name: String,
}

impl CreationValue<Element> for ElementCreationValue {
  fn new(value: Element, namespace: String, name: String) -> Self {
    Self { value, namespace, name }
  }

  fn value(&self) -> &Element {
    &self.value
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }

  fn name(&self) -> &String {
    &self.name
  }
}
