use std::io::Read;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::data::element::string::element::StringElement;
use crate::core::data::parameter::Parameters;
use crate::core::data::requirement::Requirements;
use crate::core::namespace::Namespace;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;
use crate::core::traits::operation::Operation;
use crate::core::traits::pack::Pack;
use crate::core::traits::parameter::Parameter;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug)]
pub struct StringPack {
  namespace: Namespace,
  parameters: Parameters,
  operation: Box<dyn Operation<StringElement, String>>,
}

impl StringPack {
  fn new() -> StringPack {
    StringPack {
      namespace: Namespace::empty(),
      parameters: Parameters::empty(),
      operation: Box::new(())
    }
  }
}

impl File for StringPack {
  fn suffix() -> String {
    ".string.xml".into()
  }
}

impl XmlElement for StringPack {
  fn tag_name() -> OwnedName {
    OwnedName {
      local_name: "pack".to_string(),
      namespace: Some("http://www.ato.net/xmlns/element/string".to_string()),
      prefix: Some("string".to_string())
    }
  }
}

impl YaDeserialize for StringPack {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    StringPack::peek_expect_tag_name(reader);
    reader.read_inner_value(|f| {
      let mut result = StringPack::new();
      loop {
        match reader.peek() {
          Ok(XmlEvent::StartElement) => None,
          Ok(XmlEvent::EndElement) => break Ok(result),
          Ok(_) => None,
          Err(error) => break Err(error)
        }
        reader.next_event();
      }
    })
  }
}

impl BuildableWithRequirements<StringElement, Requirements> for StringPack {
  fn build_with_requirements(&self, requirements: Requirements) -> StringElement {
    todo!()
  }
}

impl Pack<StringElement, String> for StringPack {}