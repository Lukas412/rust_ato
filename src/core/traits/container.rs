use crate::core::build::error::BuildError;
use crate::core::traits::pack::Pack;
use crate::core::traits::value::Value;
use crate::core::traits::parameter::Parameter;

pub trait Container {
  type Value: Value;
  type Parameter: Parameter;

  fn from_pack<P: Pack>(pack: P, elements: Vec<(String, Self::Value)>) -> Self;
  fn namespace(&self) -> &String;
  fn includes(&self, parameter: &Self::Parameter) -> bool;
  fn get_element(&self, name: &String) -> Option<&Self::Value>;
}

pub trait Provide<T>: Container {
  fn get(&self, name: &String, namespace: &String) -> Result<T, BuildError>;
}