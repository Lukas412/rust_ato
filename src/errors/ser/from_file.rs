use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use error_stack::Context;

#[derive(Debug)]
pub(crate) struct DeserializeFromFileError {
  file_path: PathBuf,
}

impl DeserializeFromFileError {
  pub(crate) fn new<P: AsRef<Path>>(file_path: P) -> Self {
    let file_path = file_path.as_ref().into();
    Self { file_path }
  }
}

impl Display for DeserializeFromFileError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "DeserializeError in file: '{:?}'", self.file_path)
  }
}

impl Context for DeserializeFromFileError {}