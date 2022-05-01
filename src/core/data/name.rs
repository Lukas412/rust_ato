pub struct AtoName {
  name: String,
  namespace: Option<String>,
}

impl AtoName {
  pub fn new(name: String, namespace: Option<String>) -> AtoName {
    AtoName { name, namespace }
  }
}