use crate::{BuildError, PackProvider, Requirements};
use crate::core::traits::value::Value;

pub trait Operation<V>
  where V: Value
{
  fn build(&self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<V, BuildError>;
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
  fn operation(&self, namespace: &String, name: &String) -> Result<O, BuildError>;
}