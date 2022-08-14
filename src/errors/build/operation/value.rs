use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

#[derive(Debug)]
pub(crate) struct ValueParseError {
  text: String
}

impl ValueParseError {
  pub(crate) fn new_report(text: String) -> Report<Self> {
    report!(Self::new(text))
  }

  fn new(text: String) -> Self {
    Self { text }
  }
}

impl Display for ValueParseError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while parsing value: '{}'", self.text)
  }
}

impl Context for ValueParseError {}