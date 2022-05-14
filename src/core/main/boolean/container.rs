use std::collections::HashMap;
use crate::core::build::error::BuildError;
use crate::core::main::boolean::parameter::BooleanParameter;
use crate::core::main::boolean::value::BooleanValue;
use crate::core::traits::container::Container;
use crate::core::traits::operation::ProvideOperation;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;

pub struct BooleanContainer {
  namespace: String,
  elements: HashMap<String, BooleanValue>,
}

impl Container for BooleanContainer {
  type Operation = ();
  type Parameter = BooleanParameter;

  fn from_pack<P: Pack>(pack: P, elements: Vec<(String, Self::Operation)>) -> Self {
    todo!()
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

impl Provide<BooleanValue> for BooleanContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<BooleanValue, BuildError> {
    match self.get_element(name) {
      Some(value) => Ok(value.clone()),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}