use crate::core::data::name::AtoName;
use crate::core::traits::element::Element;
use crate::core::traits::parameter::Parameter;

pub trait Container<E: Element<T>, P: Parameter, T> {
  fn satisfy_parameter(&self, parameters: &P) -> bool;
  fn get_element(&self, name: &AtoName) -> &E;
}