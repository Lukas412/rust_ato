use crate::core::main::namespace::Namespace;
use crate::core::traits::container::Container;
use crate::core::traits::operation::Operation;

pub trait Creation<T> {
  type Container: Container;
  type Value: CreationValue<T>;

  fn namespace(&self) -> &Namespace;

  fn values(&self) -> Vec<(String, Self::Value)> ;

  fn container(&self) -> Self::Container {
    <Self::Container as Container>::from(self.namespace().to_owned(), Vec::new())
  }
}

pub trait CreationValue<T> {
  fn operation<O: Operation>(&self) -> O;
}