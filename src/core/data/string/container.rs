use std::collections::HashMap;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::data::string::parameter::StringParameter;
use crate::core::data::string::value::StringValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;
use crate::core::traits::value::Value;

pub struct StringContainer {
  elements: HashMap<String, StringValue>
}

impl Container for StringContainer {
  type Value = StringValue;
  type Parameter = StringParameter;

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

impl Provide<String> for StringContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&String, Self::Error> {
    match self.get_element(name) {
      Some(value) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}