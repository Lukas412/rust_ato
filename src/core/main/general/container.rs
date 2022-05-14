use std::collections::HashMap;
use std::iter::FromIterator;
use crate::core::build::error::BuildError;
use crate::core::main::action::value::ActionValue;
use crate::core::main::boolean::operation::BooleanOperation;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::main::general::operation::CombinedGeneralOperation;

use crate::core::main::general::parameter::GeneralParameter;
use crate::core::main::general::value::{CombinedGeneralValue, GeneralValue};
use crate::core::main::number::operation::NumberOperation;
use crate::core::main::number::value::NumberValue;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::path::value::PathValue;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::container::Container;
use crate::core::traits::operation::ProvideOperation;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;
use crate::core::traits::value::Value;

pub struct GeneralContainer {
  namespace: String,
  elements: HashMap<String, CombinedGeneralOperation>,
}

impl Container for GeneralContainer {
  type Operation = CombinedGeneralOperation;
  type Parameter = GeneralParameter;

  fn from_pack<P: Pack>(pack: P, elements: Vec<(String, Self::Operation)>) -> Self {
    let namespace = pack.namespace().to_owned();
    let elements = HashMap::from_iter(elements.into_iter());
    Self { namespace, elements }
  }

  fn namespace(&self) -> &String {
    &self.namespace
  }

  fn includes(&self, parameter: &Self::Parameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&Self::Operation> {
    self.elements.get(name)
  }
}

impl ProvideOperation<BooleanOperation> for GeneralContainer {
  fn operation(&self, name: &String) -> Option<&BooleanOperation> {
    match self.get_element(name) {
      Some(CombinedGeneralOperation::Boolean(&operation)) => Some(operation),
      _ => None,
    }
  }
}

impl ProvideOperation<NumberOperation> for GeneralContainer {
  fn operation(&self, name: &String) -> Option<&NumberOperation> {
    match self.get_element(name) {
      Some(CombinedGeneralOperation::Number(&operation)) => Some(operation),
      _ => None,
    }
  }
}

impl ProvideOperation<PathOperation> for GeneralContainer {
  fn operation(&self, name: &String) -> Option<&PathOperation> {
    match self.get_element(name) {
      Some(CombinedGeneralOperation::Path(&operation)) => Some(operation),
      _ => None,
    }
  }
}

impl ProvideOperation<StringOperation> for GeneralContainer {
  fn operation(&self, name: &String) -> Option<&StringOperation> {
    match self.get_element(name) {
      Some(CombinedGeneralOperation::String(&operation)) => Some(operation),
      _ => None,
    }
  }
}