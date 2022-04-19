use std::io::Read;

use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

#[derive(Debug, Default, YaDeserialize)]
pub struct Parameters {
  #[yaserde(rename = "parameter")]
  parameters: Vec<Parameter>,
}

#[derive(Debug, Default)]
pub struct Parameter {
  name: String,
  namespace: Option<String>,
  parameter_type: ParameterType,
}

impl YaDeserialize for Parameter {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let mut result = Parameter::default();
    if let XmlEvent::StartElement { name, namespace, attributes } = reader.peek() {
      if name.local_name != "parameter" {
        return Err(format!("Expected name parameter, got {}", name.local_name));
      }
      result.parameter_type = ParameterType::from_namespace(&name.namespace)?;
    };
    reader.read_inner_value(|f| {
      loop {
        let peek = f.peek()?;
        match peek {
          XmlEvent::StartElement { .. } => {}
          XmlEvent::EndElement { .. } => {}
          XmlEvent::CData(_) => {}
          XmlEvent::Characters(_) => {}
          _ => {}
        }
        f.next_event();
      }
    })
  }
}

pub enum ParameterType {
  None,
  Boolean,
  Number,
  Path,
  String,
}

impl ParameterType {
  fn from_namespace(namespace: &Option<String>) -> Result<Self, String> {
    let actual_namespace = namespace.ok_or("parameter has to have a namespace".to_string())?;
    match actual_namespace.as_str() {
      "http://www.ato.net/xmlns/element/boolean" => Ok(Self::Boolean),
      "http://www.ato.net/xmlns/element/number" => Ok(Self::Number),
      "http://www.ato.net/xmlns/element/path" => Ok(Self::Path),
      "http://www.ato.net/xmlns/element/string" => Ok(Self::String),
      value => Err(format!("unknown namespace for parameter: {}", value))
    }
  }
}

impl Default for ParameterType {
  fn default() -> Self {
    Self::None
  }
}