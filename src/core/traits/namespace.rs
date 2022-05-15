pub type Namespace = String;

pub trait GetNamespace {
  fn get_namespace(&self) -> &Namespace;

  fn get_owned_namespace(&self) -> Namespace {
    self.get_namespace().to_owned()
  }
}