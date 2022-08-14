use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

#[derive(Debug)]
pub(crate) struct OperationNotFoundError {
  name: String
}

impl OperationNotFoundError {
  pub(crate) fn new_report(name: String) -> Report<Self> {
    report!(Self::new(name))
  }

  pub(crate) fn new(name: String) -> Self {
    Self { name }
  }
}

impl Display for OperationNotFoundError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Operation could not be found: '{}'", self.name)
  }
}

impl Context for OperationNotFoundError {}