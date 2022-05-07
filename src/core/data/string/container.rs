use std::collections::HashMap;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::data::string::parameter::StringParameter;
use crate::core::data::string::value::StringValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct StringContainer {
  namespace: String,
  elements: HashMap<String, StringValue>,
}

impl Container for StringContainer {
  type Value = StringValue;
  type Parameter = StringParameter;

  fn new(namespace: String) -> Self {
    Self {
      namespace,
      elements: HashMap::new()
    }
  }

  fn from<const N: usize>(namespace: String, elements: [(String, Self::Value); N]) -> Self {
    Self {
      namespace,
      elements: HashMap::from(elements)
    }
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }

  fn includes(&self, parameter: &Self::Parameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&Self::Value> {
    self.elements.get(name)
  }
}

impl Provide<StringValue, BuildError> for StringContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<StringValue, BuildError> {
    match self.get_element(name) {
      Some(value) => Ok(value.clone()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}