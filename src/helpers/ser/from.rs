use std::fs::File;
use std::io::Read;
use std::path::Path;

use error_stack::{IntoReport, ResultExt};
use yaserde::de::{Deserializer, from_reader};
use yaserde::YaDeserialize;

struct DeserializeFromFileError<'a, P> {
  file_path: &'a P,
}

impl<'a, P: AsRef<Path>> DeserializeFromFileError<'a, P> {
  fn new(file_path: &'a P) -> Self {
    Self { file_path }
  }
}

pub(crate) fn from_file<T: YaDeserialize, P: AsRef<Path> + ?Sized>(file_path: &P) -> error_stack::Result<T, DeserializeFromFileError<P>> {
  let file = File::open(file_path)
    .report().change_context(DeserializeFromFileError::new(file_path))?;
  from_reader(file)
    .report().change_context(DeserializeFromFileError::new(file_path))
}

pub(crate) fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}