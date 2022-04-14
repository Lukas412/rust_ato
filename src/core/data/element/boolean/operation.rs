use std::io::Read;

use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug)]
pub struct BooleanValueOperation {
  text: String,
}

impl BuildableWithRequirements<BooleanElement, String, Requirements> for BooleanValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> Result<BooleanElement, String> {
    match self.text.as_str() {
      "true" => Ok(BooleanElement::new(true)),
      "false" => Ok(BooleanElement::new(false)),
      value => Err(format!("BooleanValueOperation: ValueError: {}", value))
    }
  }
}

impl YaDeserialize for BooleanValueOperation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    BooleanValueOperation::peek_expect_tag_name(reader)?;
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

impl XmlElement for BooleanValueOperation {
  fn empty() -> Self {
    todo!()
  }

  fn tag_name() -> OwnedName {
    OwnedName {
      local_name: "value".to_string(),
      namespace: Some("http://www.ato.net/xmlns/element/boolean".to_string()),
      prefix: Some("boolean".to_string()),
    }
  }
}