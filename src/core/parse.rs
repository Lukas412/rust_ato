use std::io::Read;

use yaserde::de::Deserializer;
use crate::helpers::ser::events::start::peek_start_element;
use crate::helpers::ser::from::from_deserializer;

use crate::core::variant::{DeserializeWithVariant, Variant};

pub(crate) fn from_deserializer_with_variant<R, T>(reader: &mut Deserializer<R>) -> Result<T, String>
  where R: Read, T: DeserializeWithVariant
{
  let (name, _, _) = peek_start_element(reader)?;
  let variant = Variant::from_ownebd_name(name)?;
  let inner: T::Inner = from_deserializer(reader)?;
  T::from_inner(inner, variant)
}