use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::traits::operation::Operation;

use crate::core::data::element::path::element::PathElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug)]
pub struct PathValueOperation {
  text: String,
}

impl BuildableWithRequirements<PathElement, Requirements> for PathValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> PathElement {
    match PathBuf::from_str(&self.text) {
      Ok(value) => PathElement::new(value),
      Err(_) => PathElement::new(PathBuf::default())
    }
  }
}

impl YaDeserialize for PathValueOperation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    PathValueOperation::peek_expect_tag_name(reader)?;
    reader.read_inner_value(|reader| {
      let mut result = Self { text: "".to_string() };
      match reader.next_event()? {
        XmlEvent::Characters(text) => result.text = text,
        event => return Err(format!("Expected XmlEvent::Characters, got: {:?}", event))
      };
      return Ok(result);
    })
  }
}

impl XmlElement for PathValueOperation {
  fn tag_name() -> OwnedName {
    OwnedName {
      local_name: "value".to_string(),
      namespace: Some("http://www.ato.net/xmlns/element/path".to_string()),
      prefix: Some("path".to_string()),
    }
  }
}

impl Operation<PathElement, PathBuf> for PathValueOperation {}