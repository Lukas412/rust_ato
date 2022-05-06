use std::collections::HashMap;
use rust_decimal::Decimal;
use crate::core::data::action::value::Action;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::ElementValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;
use crate::core::traits::value::Value;

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

impl Provide<bool> for ElementContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&bool, Self::Error> {
    match self.get_element(name) {
      Some(ElementValue::Boolean(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<Decimal> for ElementContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&Decimal, Self::Error> {
    match self.get_element(name) {
      Some(ElementValue::Number(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}