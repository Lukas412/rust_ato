use std::collections::HashMap;
use crate::core::data::action::parameter::ActionParameter;
use crate::core::data::action::value::ActionValue;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct ActionContainer {
  elements: HashMap<String, ActionValue>
}

impl Container for ActionContainer {
  type Value = ActionValue;
  type Parameter = ActionParameter;

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

impl Provide<ActionValue> for ActionContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&ActionValue, Self::Error> {
    match self.get_element(name) {
      Some(value) => Ok(value),
      _ => Err(RequirementError::new(name, namespace))
    }
  }
}