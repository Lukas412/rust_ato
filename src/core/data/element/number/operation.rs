use std::io::Read;
use std::str::FromStr;

use rust_decimal::Decimal;
use rust_decimal::prelude::Zero;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;

use crate::core::data::element::number::element::NumberElement;
use crate::core::traits::operation::Operation;
use crate::core::data::requirement::Requirements;
use crate::core::traits::xml_element::XmlElement;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
pub struct NumberValueOperation {
  #[yaserde(text)]
  text: String,
}

impl BuildableWithRequirements<NumberElement, Requirements> for NumberValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> NumberElement {
    match Decimal::from_str(&self.text) {
      Ok(value) => NumberElement::new(value),
      Err(_) => NumberElement::new(Decimal::zero())
    }
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
  fn tag_name() -> OwnedName {
    OwnedName {
      local_name: "value".to_string(),
      namespace: Some("http://www.ato.net/xmlns/element/number".to_string()),
      prefix: Some("number".to_string()),
    }
  }
}

impl Operation<NumberElement, Decimal> for NumberValueOperation {}