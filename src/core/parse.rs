use std::fs::File;
use std::io::Read;
use std::path::Path;

use yaserde::__xml::attribute::OwnedAttribute;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::namespace::Namespace;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::{Deserializer, from_reader};
use yaserde::YaDeserialize;

use crate::core::variant::{DeserializeWithVariant, Variant};

pub fn from_file<T: YaDeserialize, P: AsRef<Path>>(file: P) -> Result<T, String> {
  match File::open(file) {
    Ok(file) => from_reader(file),
    Err(error) => Err(error.to_string()),
  }
}

pub fn from_deserializer<R: Read, T: YaDeserialize>(reader: &mut Deserializer<R>) -> Result<T, String> {
  <T as YaDeserialize>::deserialize(reader)
}

pub fn from_deserializer_with_variant<R, T>(reader: &mut Deserializer<R>) -> Result<T, String>
  where R: Read, T: DeserializeWithVariant
{
  let (name, _, _) = peek_start_element(reader)?;
  let variant = Variant::from_owned_name(name)?;
  let inner: T::Inner = from_deserializer(reader)?;
  T::from_inner(inner, variant)
}

pub fn peek_start_element<R: Read>(reader: &mut Deserializer<R>) -> Result<(&OwnedName, &Namespace, &Vec<OwnedAttribute>), String> {
  let peek = reader.peek()?;
  match peek {
    XmlEvent::StartElement { name, namespace, attributes } =>
      Ok((name, namespace, attributes)),
    _ => Err(format!("Expect StartElement: {:?}", peek))
  }
}