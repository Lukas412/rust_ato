use crate::BuildError;

pub enum Variant {
  Action,
  Boolean,
  Number,
  Path,
  String,
}

impl Variant {
  pub fn from_xml_namespace(xml_namespace: &str) -> Result<Self, String> {
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