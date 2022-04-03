use std::io::Read;

use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;

pub fn peek_expect_start_event<R: Read>(reader: &mut Deserializer<R>) -> Result<OwnedName, String> {
  let event = reader.peek()?;
  match event {
    XmlEvent::StartElement { name, .. } => Ok(name.to_owned()),
    _ => Err(format!("Expected XmlEvent::StartElement, got: {:?}", event))
  }
}