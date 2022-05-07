use std::collections::HashMap;
use crate::core::data::boolean::value::BooleanValue;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::ElementValue;
use crate::core::data::number::value::NumberValue;
use crate::core::data::path::value::PathValue;
use crate::core::data::string::value::StringValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct ElementContainer {
  elements: HashMap<String, ElementValue>
}

impl Container for ElementContainer {
  type Value = ElementValue;
  type Parameter = ElementParameter;

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

impl Provide<BooleanValue> for ElementContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&BooleanValue, Self::Error> {
    match self.get_element(name) {
      Some(ElementValue::Boolean(value)) => Ok(value),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<NumberValue> for ElementContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&NumberValue, Self::Error> {
    match self.get_element(name) {
      Some(ElementValue::Number(value)) => Ok(value),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<PathValue> for ElementContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&PathValue, Self::Error> {
    match self.get_element(name) {
      Some(ElementValue::Path(value)) => Ok(value),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<StringValue> for ElementContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&StringValue, Self::Error> {
    match self.get_element(name) {
      Some(ElementValue::String(value)) => Ok(value),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}