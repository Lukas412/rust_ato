use std::fs::File;
use std::io::Read;
use std::path::Path;
use yaserde::de::{Deserializer, from_reader};
use yaserde::YaDeserialize;

pub fn from_file<T: YaDeserialize, P: AsRef<Path> + ?Sized>(file: &P) -> Result<T, String> {
  match File::open(file) {
    Ok(file) => from_reader(file),
    Err(error) => Err(error.to_string()),
  }
}

pub fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}