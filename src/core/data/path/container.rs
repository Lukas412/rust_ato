use std::collections::HashMap;
use crate::core::data::path::parameter::PathParameter;
use crate::core::data::path::value::PathValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct PathContainer {
  elements: HashMap<String, PathValue>
}

impl Container<PathValue, PathParameter> for PathContainer {
  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, PathValue); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &PathParameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&PathValue> {
    self.elements.get(name)
  }
}