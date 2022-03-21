use std::iter;

use crate::core::namespace::compare_namespace;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
pub struct BooleanParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}
