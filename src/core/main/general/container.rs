use std::collections::HashMap;
use crate::core::build::error::BuildError;
use crate::core::main::action::value::ActionValue;
use crate::core::main::boolean::value::BooleanValue;

use crate::core::main::general::parameter::GeneralParameter;
use crate::core::main::general::value::{CombinedGeneralValue, GeneralValue};
use crate::core::main::number::value::NumberValue;
use crate::core::main::path::value::PathValue;
use crate::core::main::string::value::StringValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;
use crate::core::traits::value::Value;

pub struct GeneralContainer {
  namespace: String,
  elements: HashMap<String, GeneralValue>,
}

impl GeneralContainer {
  fn get_value_and_namespace(&self, name: &String, namespace: &String) -> Result<(&CombinedGeneralValue, String), BuildError> {
    match self.get_element(name) {
      Some(value) => Ok((value.value(), value.namespace().to_owned())),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
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

impl Provide<ActionValue> for GeneralContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<ActionValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedGeneralValue::Action(value), namespace) => Ok(ActionValue::new(value.to_owned(), namespace)),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<BooleanValue> for GeneralContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<BooleanValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedGeneralValue::Boolean(value), namespace) => Ok(BooleanValue::new(value.to_owned(), namespace)),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<NumberValue> for GeneralContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<NumberValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedGeneralValue::Number(value), namespace) => Ok(NumberValue::new(value.to_owned(), namespace)),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<PathValue> for GeneralContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<PathValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedGeneralValue::Path(value), namespace) => Ok(PathValue::new(value.to_owned(), namespace)),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<StringValue> for GeneralContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<StringValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedGeneralValue::String(value), namespace) => Ok(StringValue::new(value.to_owned(), namespace)),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}