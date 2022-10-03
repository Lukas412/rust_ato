use serde::Deserialize;

use crate::core::namespace::Namespace;

#[derive(Debug, Deserialize)]
#[serde(rename = "parameter")]
pub(crate) struct InnerParameter {
  name: String,
  namespace: Option<String>,
}

impl InnerParameter {
  pub(crate) fn to_name_and_optional_namespace(self) -> (String, Option<Namespace>) {
    let name = self.name;
    let optional_namespace = self.namespace.map(Namespace::new);
    (name, optional_namespace)
  }
}