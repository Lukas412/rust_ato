use std::collections::HashMap;
use std::iter::FromIterator;
use crate::core::build::error::BuildError;
use crate::core::main::number::parameter::NumberParameter;
use crate::core::main::number::value::NumberValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct NumberContainer {
  namespace: String,
  elements: HashMap<String, NumberValue>,
}

impl Container for NumberContainer {
  type Value = NumberValue;
  type Parameter = NumberParameter;

  fn new(namespace: String) -> Self {
    Self {
      namespace,
      elements: HashMap::new()
    }
  }

  fn from(namespace: String, elements: Vec<(String, Self::Value)>) -> Self {
    Self {
      namespace,
      elements: HashMap::from_iter(elements.into_iter())
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

impl Provide<NumberValue> for NumberContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<NumberValue, BuildError> {
    match self.get_element(name) {
      Some(value) => Ok(value.clone()),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}