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

pub(crate) fn from_deserializer_with_variant<R, T>(reader: &mut Deserializer<R>) -> Result<T, String>
  where R: Read, T: DeserializeWithVariant
{
  let (name, _, _) = peek_start_element(reader)?;
  let variant = Variant::from_ownebd_name(name)?;
  let inner: T::Inner = from_deserializer(reader)?;
  T::from_inner(inner, variant)
}