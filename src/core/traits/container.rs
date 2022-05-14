use crate::core::build::error::BuildError;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::Pack;
use crate::core::traits::value::Value;
use crate::core::traits::parameter::Parameter;

pub trait Container {
  type Operation: Operation;
  type Parameter: Parameter;

  fn from_pack<P: Pack>(pack: P, elements: Vec<(String, Self::Operation)>) -> Self;
  fn namespace(&self) -> &String;
  fn includes(&self, parameter: &Self::Parameter) -> bool;
  fn get_element(&self, name: &String) -> Option<&Self::Operation>;
}