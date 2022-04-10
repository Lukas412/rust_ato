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

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringValueOperation {
  text: String,
}

impl BuildableWithRequirements<StringElement, Requirements> for StringValueOperation {
  fn build_with_requirements(&self, _: Requirements) -> StringElement {
    let value = self.text.to_owned();
    StringElement::new(value)
  }
}

impl Operation<StringElement, String> for StringValueOperation {}