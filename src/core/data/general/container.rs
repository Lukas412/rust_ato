use crate::core::data::general::parameter::GeneralParameter;
use crate::core::data::general::value::GeneralValue;
use crate::core::traits::container::Container;

pub struct GeneralContainer {}

impl Container<GeneralValue, GeneralParameter> for GeneralContainer {
  fn includes(&self, parameters: &GeneralParameter) -> bool {
    todo!()
  }

  fn get_element(&self, name: &String) -> Option<&GeneralValue> {
    todo!()
  }
}