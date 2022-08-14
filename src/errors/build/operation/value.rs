use error_stack::{Report, report};

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