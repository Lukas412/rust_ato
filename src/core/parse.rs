use std::io::Read;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

pub fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}