use crate::core::data::name::AtoName;
use crate::core::traits::element::Element;
use crate::core::traits::parameter::Parameter;

pub trait Container<E: Element<T>, T> {
  fn has_parameter<P: Parameter>(&self, parameters: &P) -> bool;
  fn get_element(&self, name: &AtoName) -> &E;
}