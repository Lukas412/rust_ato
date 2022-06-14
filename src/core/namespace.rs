use std::fmt::{Display, Formatter};
use std::io::Read;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;
use yaserde::YaDeserialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Namespace(String);

impl Namespace {
  pub fn new(value: String) -> Self {
    Self(value)
  }
}

impl Default for Namespace {
  fn default() -> Self {
    Self::new("__default__".to_owned())
  }
}

impl Display for Namespace {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl YaDeserialize for Namespace {
  fn deserialize<R: Read>(reader: &mut Deserializer<R>) -> Result<Self, String> {
    let next = reader.next_event()?;
    if let XmlEvent::Characters(text) = next {
      Ok(Namespace::new(text))
    } else {
      Err(format!("Expected TextEvent: {next:?}"))
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct ParameterName(Namespace, String);

impl ParameterName {
  pub fn get_namespace(&self) -> &Namespace {
    &self.0
  }

  pub fn get_name(&self) -> &String {
    &self.1
  }
}

impl Display for ParameterName {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}.{}", self.0, self.1)
  }
}