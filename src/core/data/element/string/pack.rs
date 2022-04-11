use std::io::Read;
use yaserde::__xml::name::OwnedName;

use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::data::element::string::element::StringElement;
use crate::core::data::element::string::operation::StringOperation;
use crate::core::data::parameter::Parameters;
use crate::core::data::requirement::Requirements;
use crate::core::namespace::Namespace;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;
use crate::core::traits::pack::Pack;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug)]
pub struct StringPack {
  namespace: Namespace,
  parameters: Parameters,
  operation: StringOperation,
}

impl StringPack {
  fn new() -> StringPack {
    StringPack {
      namespace: Namespace::empty(),
      parameters: Parameters::empty(),
      operation: StringOperation::Empty
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
      let result = StringPack::new();
      loop {
        break match f.peek() {
          Ok(_) => Ok(result),
          Err(error) => Err(error)
        }
        // reader.next_event();
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