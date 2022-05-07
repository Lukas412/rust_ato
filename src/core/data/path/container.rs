use std::collections::HashMap;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::data::path::parameter::PathParameter;
use crate::core::data::path::value::PathValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct PathContainer {
  elements: HashMap<String, PathValue>
}

impl Container for PathContainer {
  type Value = PathValue;
  type Parameter = PathParameter;

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

impl Provide<PathValue> for PathContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&PathValue, Self::Error> {
    match self.get_element(name) {
      Some(value) => Ok(value),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}