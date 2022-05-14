use crate::core::main::namespace::Namespace;
use crate::core::traits::container::Container;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::{Pack, ProvidePack};
use crate::GeneralPackProvider;

pub trait Creation<T> {
  type Container: Container;
  type Value: CreationValue<T>;

  fn namespace(&self) -> &Namespace;

  fn values(&self) -> Vec<(String, Self::Value)> ;

  fn container<P: Pack>(&self, pack_provider: &GeneralPackProvider) -> Self::Container {
    let pack: P = pack_provider.pack(self.namespace())?;
    <Self::Container as Container>::from_pack(pack, Vec::new())
  }
}

pub trait CreationValue<T> {
  fn operation<O: Operation>(&self) -> O;
}