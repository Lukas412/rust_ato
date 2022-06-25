use std::fmt::{Display, Formatter};
use yaserde::__xml::name::OwnedName;
use yaserde::YaDeserialize;

#[derive(Debug, PartialEq)]
pub enum Variant {
  None,
  Action,
  Boolean,
  Number,
  Path,
  String,
}

impl Default for Variant {
  fn default() -> Self {
    Self::None
  }
}

impl Display for Variant {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Variant::None => write!(f, "None"),
      Variant::Action => write!(f, "Action"),
      Variant::Boolean => write!(f, "Boolean"),
      Variant::Number => write!(f, "Number"),
      Variant::Path => write!(f, "Path"),
      Variant::String => write!(f, "String")
    }
  }
}

impl Variant {
  pub fn from_owned_name(name: &OwnedName) -> Result<Self, String> {
    match &name.namespace {
      Some(xml_namespace) => Self::from_xml_namespace(xml_namespace),
      None => Err("NoXmlNamespace".to_owned())
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

pub trait DeserializeWithVariant
  where Self: Sized
{
  type Inner: YaDeserialize;

  fn from_inner(inner: Self::Inner, variant: Variant) -> Result<Self, String>;
}