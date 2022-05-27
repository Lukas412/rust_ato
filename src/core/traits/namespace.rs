pub type Namespace = String;

pub fn default_namespace() -> Namespace {
  "__default__".to_owned()
}

pub trait GetNamespace {
  fn get_namespace(&self) -> &Namespace;

  fn get_owned_namespace(&self) -> Namespace {
    self.get_namespace().to_owned()
  }
}