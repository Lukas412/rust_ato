use crate::{BuildError, GeneralCreationStack};
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::traits::namespace::Namespace;
use crate::core::traits::value::Value;

pub trait Operation<V>
  where V: Value
{
  fn build(&self, pack_provider: &PackProvider, requirements: &mut GeneralCreationStack) -> Result<V, BuildError>;
}

pub trait ProvideOperation<O>
  where O: Operation<Self::Value>
{
  type Value: Value;
  fn operation(&self, name: &String) -> Result<&O, BuildError>;
}

pub trait ProvideOperationWithNamespace<O>
  where O: Operation<Self::Value>
{
  type Value: Value;
  fn operation(&self, namespace: &Namespace, name: &String) -> Result<O, BuildError>;
}