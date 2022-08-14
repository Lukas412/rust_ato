use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::{Path, PathBuf};

use error_stack::{Context, IntoReport, ResultExt};
use yaserde::de::{Deserializer, from_reader};
use yaserde::YaDeserialize;
use crate::errors::ser::from_file::DeserializeFromFileError;
use crate::errors::ser::string::to_string_error_context;

pub(crate) fn from_file<T: YaDeserialize, P: AsRef<Path> + ?Sized>(file_path: &P) -> error_stack::Result<T, DeserializeFromFileError> {
  let file = File::open(file_path)
    .report().change_context(DeserializeFromFileError::new(file_path))?;
  let deserialize_result = from_reader(file);
  to_string_error_context(deserialize_result).change_context(DeserializeFromFileError::new(file_path))
}

pub(crate) fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}