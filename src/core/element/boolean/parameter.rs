use std::iter;

use crate::core::namespace::compare_namespace;
use crate::core::requirement::{Container, Requirement, Value};

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter", prefix = "boolean", namespace = "boolean: http://www.ato.net/xmlns/element/boolean")]
pub struct BooleanParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl Requirement for BooleanParameter {
  fn satisfied_by(&self, container: Container) -> bool {
    if self.namespace.is_none() {
      return true;
    }
    let container_values = container.get_values().iter().map(Value::get_namespace);
    let namespace = iter::repeat(self.namespace.as_ref().unwrap());
    iter::zip(container_values, namespace).any(compare_namespace)
  }
}