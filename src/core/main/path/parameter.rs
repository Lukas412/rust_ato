use crate::core::traits::parameter::Parameter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "path", namespace = "path: http://www.ato.net/xmlns/path")]
pub struct PathParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Parameter for PathParameter {
  fn name(&self) -> &String {
    &self.name
  }
}