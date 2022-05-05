use std::collections::HashMap;
use crate::core::data::action::parameter::ActionParameter;
use crate::core::data::action::value::ActionValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct ActionContainer {
  elements: HashMap<String, ActionValue>
}

impl Container<ActionValue, ActionParameter> for ActionContainer {
  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, ActionValue); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &ActionParameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&ActionValue> {
    self.elements.get(name)
  }
}