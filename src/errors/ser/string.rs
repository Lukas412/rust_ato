use std::fmt::{Display, Formatter};
use error_stack::{Context, IntoReport};

pub(crate) fn to_string_error_context<T>(error: Result<T, String>) -> error_stack::Result<T, StringError> {
  match error {
    Ok(value) => Ok(value),
    Err(error) => Err(StringError(error))
  }.report()
}

#[derive(Debug)]
pub(crate) struct StringError(String);

impl Display for StringError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error: '{}'", self.0)
  }
}

impl Context for StringError {}
