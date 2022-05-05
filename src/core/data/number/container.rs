use std::collections::HashMap;
use crate::core::data::number::parameter::NumberParameter;
use crate::core::data::number::value::NumberValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct NumberContainer {
  elements: HashMap<String, NumberValue>
}

impl Container<NumberValue, NumberParameter> for NumberContainer {
  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, NumberValue); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &NumberParameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&NumberValue> {
    self.elements.get(name)
  }
}