use std::collections::HashMap;
use std::path::PathBuf;
use rust_decimal::Decimal;
use crate::core::data::action::value::Action;
use crate::core::data::build::{BuildError, RequirementError};

use crate::core::data::general::parameter::GeneralParameter;
use crate::core::data::general::value::GeneralValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;
use crate::core::traits::value::Value;

pub struct GeneralContainer {
  elements: HashMap<String, GeneralValue>,
}

impl Container for GeneralContainer {
  type Value = GeneralValue;
  type Parameter = GeneralParameter;

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

impl Provide<Action> for GeneralContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&Action, Self::Error> {
    match self.get_element(name) {
      Some(GeneralValue::Action(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<bool> for GeneralContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&bool, Self::Error> {
    match self.get_element(name) {
      Some(GeneralValue::Boolean(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<Decimal> for GeneralContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&Decimal, Self::Error> {
    match self.get_element(name) {
      Some(GeneralValue::Number(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<PathBuf> for GeneralContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&PathBuf, Self::Error> {
    match self.get_element(name) {
      Some(GeneralValue::Path(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<String> for GeneralContainer {
  type Error = BuildError;

  fn get_value(&self, name: &String, namespace: &String) -> Result<&String, Self::Error> {
    match self.get_element(name) {
      Some(GeneralValue::String(value)) => Ok(value.value()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}