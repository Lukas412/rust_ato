use crate::core::traits::value::Value;
use crate::core::traits::parameter::Parameter;

pub trait Container<V: Value, P: Parameter> {
  fn includes(&self, parameters: &P) -> bool;
  fn get_element(&self, name: &String) -> Option<&V>;
}