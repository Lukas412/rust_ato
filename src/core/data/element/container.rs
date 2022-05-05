use std::collections::HashMap;
use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::ElementValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct ElementContainer {
  elements: HashMap<String, ElementValue>
}

impl ElementContainer {
  pub fn new() -> Self {
    ElementContainer { elements: HashMap::new() }
  }

  pub fn from<const N: usize>(elements: [(String, ElementValue); N]) -> Self {
    ElementContainer { elements: HashMap::from(elements) }
  }
}

impl Container<ElementValue, ElementParameter> for ElementContainer {
  fn includes(&self, parameters: &ElementParameter) -> bool {
    self.elements.contains_key(parameters.name())
  }

  fn get_element(&self, name: &String) -> Option<&ElementValue> {
    self.elements.get(name)
  }
}