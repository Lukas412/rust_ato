use crate::core::namespace::Namespace;

mod pack;

trait File {
  fn reference(&self) -> Namespace;
}