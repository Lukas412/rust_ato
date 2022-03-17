pub struct Namespace {
  value: String,
}

impl Clone for Namespace {
  fn clone(&self) -> Self {
    Namespace {
      value: self.value.clone()
    }
  }
}