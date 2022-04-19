use std::io::Read;
use yaserde::__xml::name::OwnedName;

use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::data::element::boolean::parameter::BooleanParameter;
use crate::core::data::element::number::parameter::NumberParameter;
use crate::core::data::element::path::parameter::PathParameter;
use crate::core::data::element::string::parameter::StringParameter;
use crate::core::parse::from_deserializer;

#[derive(Debug, Default, YaDeserialize)]
pub struct Parameters {
  #[yaserde(rename = "parameter")]
  parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub enum Parameter {
  Boolean(BooleanParameter),
  Number(NumberParameter),
  Path(PathParameter),
  String(StringParameter),
}

impl Parameter {
  fn from_xml_name<R: Read>(reader: &mut Deserializer<R>, name: OwnedName) -> Result<Self, String> {
    if let OwnedName { local_name, namespace: Some(namespace), .. } = name {
      match (local_name.as_str(), namespace.as_str()) {
        ("parameter", "http://www.ato.net/xmlns/element/boolean") => Ok(Parameter::Boolean(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/number") => Ok(Parameter::Number(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/path") => Ok(Parameter::Path(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/string") => Ok(Parameter::String(from_deserializer(reader)?)),
        value => Err(format!("parameter: wrong name: {:?}", value))
      }
    } else {
      Err("parameter: no namespace found".to_string())
    }
  }
}

impl YaDeserialize for Parameter {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let peek = reader.peek()?.to_owned();
    if let XmlEvent::StartElement { name, .. } = peek {
      Parameter::from_xml_name(reader, name)
    } else {
      Err(format!("parameter: wrong xml format: {:?}", peek))
    }
  }
}