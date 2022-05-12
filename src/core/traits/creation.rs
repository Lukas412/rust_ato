use crate::core::main::namespace::Namespace;
use crate::core::traits::container::Container;

pub trait Creation<T> {
  type Container: Container;
  type Value: CreationValue;

  fn namespace(&self) -> &String;

  fn values<const N: usize>(&self) -> [(String, Self::Value); N];

  fn container(&self) -> Self::Container {
    <Self::Container as Container>::from(self.namespace().to_owned(), [])
  }
}

pub trait CreationValue {}