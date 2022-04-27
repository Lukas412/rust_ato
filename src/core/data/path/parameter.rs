use crate::core::traits::parameter::Parameter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Parameter for PathParameter {
  fn name(&self) -> String {
    self.name.to_owned()
  }

  fn namespace(&self) -> String {
    self.namespace.to_owned().unwrap_or("".to_string())
  }
}