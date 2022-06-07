use std::io::Read;

use yaserde::__xml::name::OwnedName;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

use crate::core::main::general::parameter::inner::InnerParameter;
use crate::core::main::general::variant::Variant;
use crate::core::main::parse::from_deserializer;
use crate::core::main::namespace::Namespace;

pub mod inner;

#[derive(Debug, Default, YaDeserialize)]
pub struct Parameters {
  #[yaserde(rename = "parameter")]
  parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub struct Parameter {
  name: String,
  namespace: Option<Namespace>,
  variant: Variant,
}

impl Parameter {
  fn from_xml_name<R: Read>(reader: &mut Deserializer<R>, name: OwnedName) -> Result<Self, String> {
    let inner: InnerParameter = from_deserializer(reader)?;
    let variant = Variant::from_owned_name(&name)?;
    Ok(Self::from_inner(inner, variant))
  }

  fn from_inner(inner: InnerParameter, variant: Variant) -> Self {
    let (name, namespace) = inner.to_name_and_optional_namespace();
    Self { name, namespace, variant }
  }
}

impl YaDeserialize for Parameter {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let peek = reader.peek()?.to_owned();
    if let XmlEvent::StartElement { name, .. } = peek {
      Parameter::from_xml_name(reader, name)
    } else {
      Err(format!("ExpectStartElement: {:?}", peek))
    }
  }
}