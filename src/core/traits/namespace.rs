pub type Namespace = String;

pub trait NamespaceDefault {
  fn default() -> Self;
}

impl NamespaceDefault for Namespace {
  fn default() -> Self {
    "__default__".to_owned()
  }
}

pub trait GetNamespace {
  fn get_namespace(&self) -> &Namespace;

  fn get_owned_namespace(&self) -> Namespace {
    self.get_namespace().to_owned()
  }
}