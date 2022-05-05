use std::collections::HashMap;

use crate::core::data::general::parameter::GeneralParameter;
use crate::core::data::general::value::GeneralValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct GeneralContainer {
  elements: HashMap<String, GeneralValue>,
}

impl Container<GeneralValue, GeneralParameter> for GeneralContainer {
  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, GeneralValue); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &GeneralParameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&GeneralValue> {
    todo!()
  }
}