use crate::core::traits::namespace::Namespace;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::{Pack, ProvidePack};
use crate::GeneralPackProvider;

pub trait Creation<O>
  where
    O: Operation,
    GeneralPackProvider: ProvidePack<<Self as Creation<O>>::Pack>
{
  type Pack: Pack;
  type Value: CreationValue<O>;

  fn namespace(&self) -> &Namespace;

  fn values(&self) -> Vec<(String, Self::Value)>;
}

pub trait CreationValue<O>
  where O: Operation
{
  fn to_operation(self) -> O;
}