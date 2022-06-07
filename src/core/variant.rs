use std::io::Read;

use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::parse::from_deserializer;

#[derive(Debug)]
pub enum Variant {
  Action,
  Boolean,
  Number,
  Path,
  String,
}

impl Variant {
  pub fn from_owned_name(name: &OwnedName) -> Result<Self, String> {
    match &name.namespace {
      Some(xml_namespace) => Self::from_xml_namespace(xml_namespace),
      None => Err(format!("NoXmlNamespace"))
    }
  }

  fn from_xml_namespace(xml_namespace: &str) -> Result<Self, String> {
    match xml_namespace {
      "http://www.ato.net/xmlns/action" => Ok(Self::Action),
      "http://www.ato.net/xmlns/boolean" => Ok(Self::Boolean),
      "http://www.ato.net/xmlns/number" => Ok(Self::Number),
      "http://www.ato.net/xmlns/path" => Ok(Self::Path),
      "http://www.ato.net/xmlns/string" => Ok(Self::String),
      _ => Err(format!("UnknownXmlNamespace: '{}'", xml_namespace))
    }
  }
}

pub trait DeserializeWithVariant<I: YaDeserialize> {
  fn from_deserializer<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let peek = reader.peek()?;
    if let XmlEvent::StartElement { name, .. } = peek {
      let variant = Variant::from_owned_name(name)?;
      let inner: I = from_deserializer(reader)?;
      Self::from_inner(inner, variant)
    } else {
      Err(format!("ExpectStartElement: {:?}", peek))
    }
  }

  fn from_inner(inner: I, variant: Variant) -> Result<Self, String>;
}