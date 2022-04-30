use std::io::Read;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;
use crate::core::data::action::parameter::ActionParameter;
use crate::core::data::boolean::parameter::BooleanParameter;
use crate::core::data::number::parameter::NumberParameter;
use crate::core::data::path::parameter::PathParameter;
use crate::core::data::string::parameter::StringParameter;
use crate::core::parse::from_deserializer;
use crate::core::traits::parameter::Parameter;

#[derive(Debug, Default, YaDeserialize)]
pub struct ElementParameters {
  #[yaserde(rename = "parameter")]
  parameters: Vec<GeneralParameter>,
}

#[derive(Debug)]
pub enum GeneralParameter {
  Action(ActionParameter),
  Boolean(BooleanParameter),
  Number(NumberParameter),
  Path(PathParameter),
  String(StringParameter),
}

impl GeneralParameter {
  fn from_xml_name<R: Read>(reader: &mut Deserializer<R>, name: OwnedName) -> Result<Self, String> {
    if let OwnedName { local_name, namespace: Some(namespace), .. } = name {
      match (local_name.as_str(), namespace.as_str()) {
        ("parameter", "http://www.ato.net/xmlns/action") => Ok(GeneralParameter::Action(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/boolean") => Ok(GeneralParameter::Boolean(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/number") => Ok(GeneralParameter::Number(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/path") => Ok(GeneralParameter::Path(from_deserializer(reader)?)),
        ("parameter", "http://www.ato.net/xmlns/element/string") => Ok(GeneralParameter::String(from_deserializer(reader)?)),
        value => Err(format!("parameter: wrong name: {:?}", value))
      }
    } else {
      Err("parameter: no namespace found".to_string())
    }
  }
}

impl YaDeserialize for GeneralParameter {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let peek = reader.peek()?.to_owned();
    if let XmlEvent::StartElement { name, .. } = peek {
      GeneralParameter::from_xml_name(reader, name)
    } else {
      Err(format!("parameter: wrong xml format: {:?}", peek))
    }
  }
}

impl Parameter for GeneralParameter {
  fn name(&self) -> String {
    match self {
      GeneralParameter::Boolean(parameter) => parameter.name(),
      GeneralParameter::Number(parameter) => parameter.name(),
      GeneralParameter::Path(parameter) => parameter.name(),
      GeneralParameter::String(parameter) => parameter.name(),
    }
  }

  fn namespace(&self) -> String {
    match self {
      GeneralParameter::Boolean(parameter) => parameter.namespace(),
      GeneralParameter::Number(parameter) => parameter.namespace(),
      GeneralParameter::Path(parameter) => parameter.namespace(),
      GeneralParameter::String(parameter) => parameter.namespace(),
    }
  }
}