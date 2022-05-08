use std::collections::HashMap;

use crate::core::main::boolean::value::BooleanValue;
use crate::core::main::build::{BuildError, RequirementError};
use crate::core::main::element::parameter::ElementParameter;
use crate::core::main::element::value::{CombinedElementValue, ElementValue};
use crate::core::main::number::value::NumberValue;
use crate::core::main::path::value::PathValue;
use crate::core::main::string::value::StringValue;
use crate::core::traits::container::{Container, Provide};
use crate::core::traits::parameter::Parameter;
use crate::core::traits::value::Value;

pub struct ElementContainer {
  namespace: String,
  elements: HashMap<String, ElementValue>,
}

impl ElementContainer {
  fn get_value_and_namespace(&self, name: &String, namespace: &String) -> Result<(&CombinedElementValue, String), BuildError> {
    match self.get_element(name) {
      Some(value) => Ok((value.value(), value.namespace().to_owned())),
      _ => Err(RequirementError::new(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Container for ElementContainer {
  type Value = ElementValue;
  type Parameter = ElementParameter;

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

impl Provide<BooleanValue, BuildError> for ElementContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<BooleanValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedElementValue::Boolean(value), namespace) => Ok(BooleanValue::new(value.to_owned(), namespace)),
      _ => Err(RequirementError::new(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<NumberValue, BuildError> for ElementContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<NumberValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedElementValue::Number(value), namespace) => Ok(NumberValue::new(value.to_owned(), namespace)),
      _ => Err(RequirementError::new(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<PathValue, BuildError> for ElementContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<PathValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedElementValue::Path(value), namespace) => Ok(PathValue::new(value.to_owned(), namespace)),
      _ => Err(RequirementError::new(name.to_owned(), namespace.to_owned())),
    }
  }
}

impl Provide<StringValue, BuildError> for ElementContainer {
  fn get_value(&self, name: &String, namespace: &String) -> Result<StringValue, BuildError> {
    match self.get_value_and_namespace(name, namespace)? {
      (CombinedElementValue::String(value), namespace) => Ok(StringValue::new(value.to_owned(), namespace)),
      _ => Err(RequirementError::new(name.to_owned(), namespace.to_owned())),
    }
  }
}