use std::collections::HashMap;
use std::iter::FromIterator;
use crate::core::build::error::BuildError;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::path::parameter::PathParameter;
use crate::core::main::path::value::PathValue;
use crate::core::traits::container::Container;
use crate::core::traits::operation::ProvideOperation;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;

pub struct PathContainer {
  namespace: String,
  elements: HashMap<String, PathOperation>,
}

impl Container for PathContainer {
  type Operation = PathOperation;
  type Parameter = PathParameter;

  fn from_pack<P: Pack>(pack: P, elements: Vec<(String, Self::Operation)>) -> Self {
    Self {
      namespace: pack.namespace().to_owned(),
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

impl ProvideOperation<PathOperation> for PathContainer {
  fn operation(&self, name: &String) -> Option<&PathOperation> {
    self.get_element(name)
  }
}