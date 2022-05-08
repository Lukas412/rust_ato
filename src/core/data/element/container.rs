use std::collections::HashMap;

use crate::core::data::boolean::value::BooleanValue;
use crate::core::data::build::{BuildError, RequirementError};
use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::{CombinedElementValue, ElementValue};
use crate::core::data::number::value::NumberValue;
use crate::core::data::path::value::PathValue;
use crate::core::data::string::value::StringValue;
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