pub struct Container {
  pub values: Vec<Value>,
}

impl Container {
  pub fn get_values(&self) -> &Vec<Value> {
    &self.values
  }
}

pub struct Value {
  namespace: String,
  name: String,
}

impl Value {
  pub fn get_namespace(&self) -> &String {
    &self.namespace
  }
  pub fn get_name(&self) -> &str {
    &self.name
  }
}

pub trait Requirement {
  fn satisfied_by(&self, container: Container) -> bool;
}