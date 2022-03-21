use std::iter;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "path", namespace = "path: http://www.ato.net/xmlns/element/path")]
pub struct PathParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}
