use std::collections::HashMap;

use crate::core::data::general::parameter::GeneralParameter;
use crate::core::data::general::value::GeneralValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct GeneralContainer {
  elements: HashMap<String, GeneralValue>,
}

impl Container for GeneralContainer {
  type Value = GeneralValue;
  type Parameter = GeneralParameter;

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