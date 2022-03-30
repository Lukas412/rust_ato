use crate::core::data::element::parameter::Parameter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "number", namespace = "number: http://www.ato.net/xmlns/element/number")]
pub struct NumberParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Parameter for NumberParameter {}