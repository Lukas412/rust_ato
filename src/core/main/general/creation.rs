use crate::core::main::general::value::GeneralValue;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "element", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
pub struct GeneralCreation {
  #[yaserde(attribute)]
  namespace: String,
  #[yaserde(rename = "value")]
  values: Vec<GeneralCreationValue>,
}

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "value")]
pub struct GeneralCreationValue {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  value: Option<String>,
  #[yaserde(rename = "element", prefix = "creation", namespace = "creation: http://www.ato.net/xmlns/creation")]
  elements: Vec<GeneralCreation>,
}