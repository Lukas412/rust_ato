use crate::core::main::namespace::Namespace;

#[derive(Debug, YaDeserialize)]
#[yaserde(rename = "parameter")]
pub struct InnerParameter {
  #[yaserde(attribute)]
  name: String,
  #[yaserde(attribute)]
  namespace: Option<String>,
}

impl InnerParameter {
  pub fn to_name_and_optional_namespace(self) -> (String, Option<Namespace>) {
    let name = self.name;
    let optional_namespace = self.namespace.map(Namespace::new);
    (name, optional_namespace)
  }
}