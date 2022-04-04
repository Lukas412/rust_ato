use std::io::Read;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::traits::operation::Operation;
use crate::core::data::element::string::element::StringElement;
use crate::core::data::requirement::Requirements;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug)]
pub struct StringValueOperation {
  text: String,
}

impl BuildableWithRequirements<StringElement, Requirements> for StringValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}

impl YaDeserialize for StringValueOperation {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    StringValueOperation::peek_expect_tag_name(reader)?;
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

impl XmlElement for StringValueOperation {
  fn tag_name() -> OwnedName {
    OwnedName {
      local_name: "value".to_string(),
      namespace: Some("http://www.ato.net/xmlns/element/string".to_string()),
      prefix: Some("string".to_string()),
    }
  }
}


impl Operation<StringElement, String> for StringValueOperation {}