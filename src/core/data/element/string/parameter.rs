use crate::core::data::element::Parameter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "string", namespace = "string: http://www.ato.net/xmlns/element/string")]
pub struct StringParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Parameter for StringParameter {}