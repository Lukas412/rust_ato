use std::io::Read;

use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::parse::peek_expect_start_event;

pub trait XmlElement: YaDeserialize {
  fn empty() -> Self;
  fn tag_name() -> OwnedName;
  fn on_event<R: Read>(&mut self, reader: &mut Deserializer<R>) -> Result<(), String> {
    reader.next_event()?;
    Ok(())
  }

  fn peek_expect_tag_name<R: Read>(reader: &mut Deserializer<R>) -> Result<(), String> {
    let expected_tag_name = Self::tag_name();
    let read_tag_name = peek_expect_start_event(reader)?;
    if read_tag_name == expected_tag_name {
      Ok(())
    } else {
      Err(format!("Expected {:?}, got: {:?}", expected_tag_name, read_tag_name))
    }
  }

  fn read_inner_element<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    reader.read_inner_value(Self::read_inner_value)
  }

  fn read_inner_value<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let mut result = Self::empty();
    loop {
      let peek_event = reader.peek()?;
      match peek_event {
        XmlEvent::EndElement { .. } => break Ok(result),
        _ => result.on_event(reader)?,
      }
    }
  }
}