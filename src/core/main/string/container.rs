use std::collections::HashMap;
use std::iter::FromIterator;
use crate::core::build::error::BuildError;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::parameter::StringParameter;
use crate::core::main::string::value::StringValue;
use crate::core::traits::container::Container;
use crate::core::traits::operation::ProvideOperation;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;

pub struct StringContainer {
  namespace: String,
  elements: HashMap<String, StringOperation>,
}

impl Container for StringContainer {
  type Operation = StringOperation;
  type Parameter = StringParameter;

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

  fn get_element(&self, name: &String) -> Option<&Self::Operation> {
    self.elements.get(name)
  }
}

impl ProvideOperation<StringOperation> for StringContainer {
  fn operation(&self, name: &String) -> Option<&StringOperation> {
    self.get_element(name)
  }
}