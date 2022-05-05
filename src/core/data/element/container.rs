use std::collections::HashMap;
use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::ElementValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct ElementContainer {
  elements: HashMap<String, ElementValue>
}

impl Container<ElementValue, ElementParameter> for ElementContainer {
  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, ElementValue); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &ElementParameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&ElementValue> {
    self.elements.get(name)
  }
}