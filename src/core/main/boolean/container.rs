use std::collections::HashMap;
use crate::core::build::error::BuildError;
use crate::core::main::boolean::parameter::BooleanParameter;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct BooleanContainer {
  namespace: String,
  elements: HashMap<String, BooleanValue>,
}

impl Container for BooleanContainer {
  type Value = BooleanValue;
  type Parameter = BooleanParameter;

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

impl Provide<BooleanValue, BuildError> for BooleanContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<BooleanValue, BuildError> {
    match self.get_element(name) {
      Some(value) => Ok(value.clone()),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}