use crate::core::traits::value::Value;
use crate::core::traits::parameter::Parameter;

pub trait Container {
  type Value: Value;
  type Parameter: Parameter;

  fn new() -> Self;
  fn from<const N: usize>(elements: [(String, Self::Value); N]) -> Self;
  fn includes(&self, parameter: &Self::Parameter) -> bool;
  fn get_element(&self, name: &String) -> Option<&Self::Value>;
}

pub trait Provide<T, Error>: Container {
  fn get_value(&self, name: &String) -> Result<&T, Error>;
}