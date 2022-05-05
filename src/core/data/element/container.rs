use crate::core::data::element::parameter::ElementParameter;
use crate::core::data::element::value::ElementValue;
use crate::core::traits::container::Container;

pub struct ElementContainer {

}

impl Container<ElementValue, ElementParameter> for ElementContainer {
  fn includes(&self, parameters: &ElementParameter) -> bool {
    todo!()
  }

  fn get_element(&self, name: &String, namespace: Option<String>) -> &ElementValue {
    todo!()
  }
}