use std::io::Read;
use yaserde::__xml::name::OwnedName;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::parse::peek_expect_start_event;

pub trait XmlElement: YaDeserialize {
  fn tag_name() -> OwnedName;
  fn peek_expect_tag_name<R: Read>(reader: &mut Deserializer<R>) -> Result<(), String> {
    let expected_tag_name = Self::tag_name();
    let read_tag_name = peek_expect_start_event(reader)?;
    if read_tag_name == expected_tag_name {
      Ok(())
    } else {
      Err(format!("Expected {:?}, got: {:?}", expected_tag_name, read_tag_name))
    }
  }
}