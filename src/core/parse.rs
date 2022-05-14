use std::fs::{File, read_to_string};
use std::io;
use std::io::Read;

use yaserde::de::{Deserializer, from_reader, from_str};
use yaserde::YaDeserialize;

pub fn from_file<T: YaDeserialize>(file: &str) -> Result<T, String> {
  match File::open(file) {
    Ok(file) => from_reader(file),
    Err(error) => Err(error.to_string()),
  }
}

pub fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}