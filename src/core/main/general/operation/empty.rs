use crate::core::build::error::BuildError;
use crate::core::traits::namespace::GetNamespace;
use crate::core::traits::value::Value;

pub fn build_empty<R, V>(requirements: &R) -> Result<V, BuildError>
  where
    R: GetNamespace,
    V: Value
{
  let namespace = requirements.get_owned_namespace();
  Ok(V::default(namespace))
}