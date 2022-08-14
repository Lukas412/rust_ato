use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

#[derive(Debug, Default)]
pub(crate) struct CreationStackEmptyError {}

impl CreationStackEmptyError {
  pub(crate) fn new_report() -> Report<Self> {
    report!(Self::default())
  }
}

impl Display for CreationStackEmptyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "The creation stack is empty")
  }
}

impl Context for CreationStackEmptyError {}