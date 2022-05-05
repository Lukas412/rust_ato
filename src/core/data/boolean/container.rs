use std::collections::HashMap;
use crate::core::data::boolean::parameter::BooleanParameter;
use crate::core::data::boolean::value::BooleanValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct BooleanContainer {
  elements: HashMap<String, BooleanValue>
}

impl Container for BooleanContainer {
  type Value = BooleanValue;
  type Parameter = BooleanParameter;

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