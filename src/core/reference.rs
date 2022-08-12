use std::io::Read;
use yaserde::__xml::attribute::OwnedAttribute;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "references")]
pub(crate) struct GeneralReferences {
  #[yaserde(rename = "reference")]
  references: Vec<GeneralReference>,
}

#[derive(Debug)]
pub(crate) enum GeneralReference {
  Action(String),
  Boolean(String),
  Number(String),
  Path(String),
  String(String),
}

impl GeneralReference {
  fn from_xml_name_and_attributes(name: OwnedName, attributes: Vec<OwnedAttribute>) -> Result<Self, String> {
    if let OwnedName { local_name, namespace: Some(namespace), .. } = name {
      if local_name != "reference" {
        Err(format!("reference: wrong name: {:?}", local_name))
      } else {
        let namespace_attribute = Self::get_namespace_attribute_value(attributes)?;
        match namespace.as_str() {
          "http://www.ato.net/xmlns/action" => Ok(GeneralReference::Action(namespace_attribute)),
          "http://www.ato.net/xmlns/boolean" => Ok(GeneralReference::Boolean(namespace_attribute)),
          "http://www.ato.net/xmlns/number" => Ok(GeneralReference::Number(namespace_attribute)),
          "http://www.ato.net/xmlns/path" => Ok(GeneralReference::Path(namespace_attribute)),
          "http://www.ato.net/xmlns/string" => Ok(GeneralReference::String(namespace_attribute)),
          value => Err(format!("reference: wrong namespace: {:?}", value))
        }
      }
    } else {
      Err("reference: no namespace found".to_owned())
    }
  }

  fn get_namespace_attribute_value(attributes: Vec<OwnedAttribute>) -> Result<String, String> {
    for attribute in attributes {
      if attribute.name.local_name == "namespace" {
        return Ok(attribute.value);
      }
    }
    Err("reference: namespace attribute missing".to_owned())
  }
}

impl YaDeserialize for GeneralReference {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let peek = reader.peek()?.to_owned();
    if let XmlEvent::StartElement { name, attributes, .. } = peek {
      GeneralReference::from_xml_name_and_attributes(name, attributes)
    } else {
      Err(format!("parameter: wrong xml format: {:?}", peek))
    }
  }
}