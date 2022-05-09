use std::io::Read;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::main::boolean::parameter::BooleanParameter;
use crate::core::main::number::parameter::NumberParameter;
use crate::core::main::path::parameter::PathParameter;
use crate::core::main::string::parameter::StringParameter;
use crate::core::parse::from_deserializer;
use crate::core::traits::parameter::Parameter;

#[derive(Debug, Default, YaDeserialize)]
pub struct ElementParameters {
  #[yaserde(rename = "parameter")]
  parameters: Vec<ElementParameter>,
}

#[derive(Debug)]
pub enum ElementParameter {
  Boolean(BooleanParameter),
  Number(NumberParameter),
  Path(PathParameter),
  String(StringParameter),
}

impl ElementParameter {
  fn from_xml_name<R: Read>(reader: &mut Deserializer<R>, name: OwnedName) -> Result<Self, String> {
    if let OwnedName { local_name, namespace: Some(namespace), .. } = name {
      match (local_name.as_str(), namespace.as_str()) {
        ("parameter", "http://www.ato.net/xmlns/boolean") => Ok(ElementParameter::Boolean(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/number") => Ok(ElementParameter::Number(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/path") => Ok(ElementParameter::Path(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/string") => Ok(ElementParameter::String(from_deserializer(reader)?)),
        value => Err(format!("parameter: wrong name: {:?}", value))
      }
    } else {
      Err("parameter: no namespace found".to_owned())
    }
  }
}

impl YaDeserialize for ElementParameter {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let peek = reader.peek()?.to_owned();
    if let XmlEvent::StartElement { name, .. } = peek {
      ElementParameter::from_xml_name(reader, name)
    } else {
      Err(format!("parameter: wrong xml format: {:?}", peek))
    }
  }
}

impl Parameter for ElementParameter {
  fn name(&self) -> &String {
    match self {
      ElementParameter::Boolean(parameter) => parameter.name(),
      ElementParameter::Number(parameter) => parameter.name(),
      ElementParameter::Path(parameter) => parameter.name(),
      ElementParameter::String(parameter) => parameter.name(),
    }
  }
}