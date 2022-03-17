use crate::core::file::File;
use crate::core::namespace::Namespace;

struct Pack {
  namespace: Namespace
}

impl File for Pack {
  fn reference(&self) -> Namespace {
    self.namespace.to_owned()
  }
}