use std::collections::HashMap;
use crate::core::data::string::parameter::StringParameter;
use crate::core::data::string::value::StringValue;
use crate::core::traits::container::Container;
use crate::core::traits::parameter::Parameter;

pub struct StringContainer {
  elements: HashMap<String, StringValue>
}

impl Container<StringValue, StringParameter> for StringContainer {
  fn new() -> Self {
    Self { elements: HashMap::new() }
  }

  fn from<const N: usize>(elements: [(String, StringValue); N]) -> Self {
    Self { elements: HashMap::from(elements) }
  }

  fn includes(&self, parameter: &StringParameter) -> bool {
    self.elements.contains_key(parameter.name())
  }

  fn get_element(&self, name: &String) -> Option<&StringValue> {
    self.elements.get(name)
  }
}