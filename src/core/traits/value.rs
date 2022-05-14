use crate::core::main::namespace::Namespace;

pub trait Value {
  type Type;

  fn default(namespace: Namespace) -> Self;

  fn new(value: Self::Type, namespace: Namespace) -> Self;

  fn value(&self) -> &Self::Type;

  fn namespace(&self) -> &Namespace;
}