use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
struct BuildError {}

impl Display for BuildError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while building")
  }
}

impl Context for BuildError {}