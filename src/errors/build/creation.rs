use std::fmt::{Display, Formatter, write};
use error_stack::{Context, Report, report};
use crate::core::namespace::Namespace;

#[derive(Debug)]
pub(crate) struct CreationNotFoundError {
  namespace: Namespace
}

impl CreationNotFoundError {
  pub(crate) fn new_report(namespace: Namespace) -> Report<Self> {
    report!(Self::new(namespace))
  }

  fn new(namespace: Namespace) -> Self {
    Self { namespace }
  }
}

impl Display for CreationNotFoundError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Creation for namespace not found: '{}'", self.namespace)
  }
}

impl Context for CreationNotFoundError {}