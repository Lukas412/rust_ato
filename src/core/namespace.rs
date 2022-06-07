use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct Namespace(String);

impl Namespace {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}

impl Default for Namespace {
  fn default() -> Self {
    Self::new("__default__".to_owned())
  }
}

impl Display for Namespace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}