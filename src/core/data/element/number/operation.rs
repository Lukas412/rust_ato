use std::io::Read;
use std::str::FromStr;

use rust_decimal::Decimal;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::data::build::BuildError;
use crate::core::data::element::number::element::NumberElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug)]
pub struct NumberValueOperation {
  text: String,
}

impl BuildableWithRequirements<NumberElement, BuildError, Requirements> for NumberValueOperation {
  fn build_with_requirements(&self, _: &Requirements) -> Result<NumberElement, BuildError> {
    NumberElement::from_str(&self.text)
  }
}

impl YaDeserialize for NumberValueOperation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    NumberValueOperation::peek_expect_tag_name(reader)?;
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

impl XmlElement for NumberValueOperation {
  fn empty() -> Self {
    todo!()
  }

  fn tag_name() -> OwnedName {
    OwnedName {
      local_name: "value".to_string(),
      namespace: Some("http://www.ato.net/xmlns/element/number".to_string()),
      prefix: Some("number".to_string()),
    }
  }
}