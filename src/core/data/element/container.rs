use std::collections::HashMap;
use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::ElementValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct ElementContainer {
  elements: HashMap<String, ElementValue>
}

impl Container for ElementContainer {
  type Value = ElementValue;
  type Parameter = ElementParameter;

  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, Self::Value); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &Self::Parameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&Self::Value> {
    self.elements.get(name)
  }
}