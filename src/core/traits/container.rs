use crate::core::traits::value::Value;
use crate::core::traits::parameter::Parameter;

pub trait Container<V: Value, P: Parameter> {
  fn new() -> Self;
  fn from<const N: usize>(elements: [(String, V); N]) -> Self;
  fn includes(&self, parameters: &P) -> bool;
  fn get_element(&self, name: &String) -> Option<&V>;
}