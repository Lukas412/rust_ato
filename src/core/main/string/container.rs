use std::collections::HashMap;
use std::iter::FromIterator;
use crate::core::build::error::BuildError;
use crate::core::main::string::parameter::StringParameter;
use crate::core::main::string::value::StringValue;
use crate::core::traits::container::Container;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;

pub struct StringContainer {
  namespace: String,
  elements: HashMap<String, StringValue>,
}

impl Container for StringContainer {
  type Operation = ();
  type Parameter = StringParameter;

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

impl Provide<StringValue> for StringContainer {
  fn get(&self, name: &String, namespace: &String) -> Result<StringValue, BuildError> {
    match self.get_element(name) {
      Some(value) => Ok(value.clone()),
      _ => Err(BuildError::new_requirement(name.to_owned(), namespace.to_owned())),
    }
  }
}