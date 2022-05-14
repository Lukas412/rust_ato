use crate::core::build::error::BuildError;
use crate::core::traits::container::Container;
use crate::core::traits::value::Value;

pub fn build_empty<C: Container, V: Value>(requirements: &C) -> Result<V, BuildError> {
  let namespace = requirements.namespace().to_owned();
  Ok(V::default(namespace))
}