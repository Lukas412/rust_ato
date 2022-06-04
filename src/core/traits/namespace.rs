use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Namespace(String);

impl Default for Namespace {
  fn default() -> Self {
    Self("__default__".to_owned())
  }
}

impl Display for Namespace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub trait GetNamespace {
  fn get_namespace(&self) -> &Namespace;

  fn get_owned_namespace(&self) -> Namespace {
    self.get_namespace().to_owned()
  }
}