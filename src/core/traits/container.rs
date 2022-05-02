use crate::core::traits::element::Value;
use crate::core::traits::parameter::Parameter;

pub trait Container<E: Value<T>, P: Parameter, T> {
  fn satisfy_parameter(&self, parameters: &P) -> bool;
  fn get_element(&self, name: &String, namespace: Option<String>) -> &E;
}