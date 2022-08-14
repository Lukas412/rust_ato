use std::fmt::{Display, Formatter};
use crate::core::namespace::Namespace;

#[derive(Debug)]
pub(crate) struct NamespaceInformation(Namespace);

impl NamespaceInformation {
  pub(crate) const fn new(namespace: Namespace) -> Self {
    Self(namespace)
  }
}

impl Display for NamespaceInformation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "In namespace: '{}'", self.0)
  }
}