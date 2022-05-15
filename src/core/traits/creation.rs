use crate::core::build::error::BuildError;
use crate::core::main::namespace::Namespace;
use crate::core::traits::container::Container;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::{Pack, ProvidePack};
use crate::GeneralPackProvider;

pub trait Creation<T>
  where GeneralPackProvider: ProvidePack<<Self as Creation<T>>::Pack>
{
  type Container: Container;
  type Pack: Pack;
  type Value: CreationValue<T>;

  fn namespace(&self) -> &Namespace;

  fn values(&self) -> Vec<(String, Self::Value)> ;

  fn container(&self, pack_provider: &GeneralPackProvider) -> Result<Self::Container, BuildError> {
    let pack: &Self::Pack = pack_provider.pack(self.namespace())?;
    Ok(<Self::Container as Container>::from_pack::<Self::Pack>(pack, Vec::new()))
  }
}

pub trait CreationValue<T> {
  fn operation<O: Operation>(&self) -> O;
}