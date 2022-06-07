use crate::{BuildError, CreationStack};
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::traits::namespace::Namespace;
use crate::core::traits::value::Value;

pub trait Operation<V>
  where V: Value
{
  fn build(&self, pack_provider: &PackProvider, requirements: &mut CreationStack) -> Result<V, BuildError>;
}