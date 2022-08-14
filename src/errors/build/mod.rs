pub(crate) mod operation;

use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub(crate) struct BuildError {}

impl Display for BuildError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error occurred while building")
  }
}

impl Context for BuildError {}