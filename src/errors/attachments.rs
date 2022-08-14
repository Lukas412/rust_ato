use std::fmt::{Display, Formatter};
use crate::core::namespace::Namespace;
use crate::core::variant::Variant;

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

#[derive(Debug)]
pub(crate) struct VariantInformation(Variant);

impl VariantInformation {
  pub(crate) const fn new(variant: Variant) -> Self {
    Self(variant)
  }
}

impl Display for VariantInformation {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "With variant: '{}'", self.0)
  }
}