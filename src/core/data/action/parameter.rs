use crate::core::traits::parameter::Parameter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "action", namespace = "action: http://www.ato.net/xmlns/action")]
pub struct ActionParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Parameter for ActionParameter {
  fn name(&self) -> &String {
    &self.name
  }
}