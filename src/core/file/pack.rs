use crate::core::file::File;

struct Pack {
  namespace: String,
}

impl File for Pack {
  fn reference(&self) -> String {
    self.namespace.to_owned()
  }
}