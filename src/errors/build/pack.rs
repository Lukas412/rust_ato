use std::fmt::{Display, Formatter};
use error_stack::{Context, report, Report};
use crate::core::namespace::Namespace;

#[derive(Debug)]
pub(crate) struct PackNotFoundError {
  namespace: Namespace
}

impl PackNotFoundError {
  pub(crate) fn new_record(namespace: Namespace) -> Report<Self> {
    report!(Self::new(namespace))
  }

  fn new(namespace: Namespace) -> Self {
    Self { namespace }
  }
}

impl Display for PackNotFoundError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Pack could not be found: {}", self.namespace)
  }
}

impl Context for PackNotFoundError {}