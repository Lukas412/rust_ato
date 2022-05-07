use std::collections::HashMap;
use crate::core::data::action::value::ActionValue;
use crate::core::data::boolean::value::BooleanValue;
use crate::core::data::build::{BuildError, RequirementError};

use crate::core::data::general::parameter::GeneralParameter;
use crate::core::data::general::value::GeneralValue;
use crate::core::data::number::value::NumberValue;
use crate::core::data::path::value::PathValue;
use crate::core::data::string::value::StringValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;

pub struct GeneralContainer {
  namespace: String,
  elements: HashMap<String, GeneralValue>,
}

impl Container for GeneralContainer {
  type Value = GeneralValue;
  type Parameter = GeneralParameter;

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

impl Provide<ActionValue, BuildError> for GeneralContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<ActionValue, BuildError> {
    match self.get_element(name) {
      Some(GeneralValue::Action(value)) => Ok(value.clone()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<BooleanValue, BuildError> for GeneralContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<BooleanValue, BuildError> {
    match self.get_element(name) {
      Some(GeneralValue::Boolean(value)) => Ok(value.clone()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<NumberValue, BuildError> for GeneralContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<NumberValue, BuildError> {
    match self.get_element(name) {
      Some(GeneralValue::Number(value)) => Ok(value.clone()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<PathValue, BuildError> for GeneralContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<PathValue, BuildError> {
    match self.get_element(name) {
      Some(GeneralValue::Path(value)) => Ok(value.clone()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}

impl Provide<StringValue, BuildError> for GeneralContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<StringValue, BuildError> {
    match self.get_element(name) {
      Some(GeneralValue::String(value)) => Ok(value.clone()),
      _ => Err(RequirementError::new(name, namespace)),
    }
  }
}