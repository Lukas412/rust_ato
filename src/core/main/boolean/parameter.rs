use crate::core::traits::parameter::Parameter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
pub struct BooleanParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Parameter for BooleanParameter {
  fn name(&self) -> &String {
    &self.name
  }
}