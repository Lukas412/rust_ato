pub trait Operation {
  type Value;
}

pub trait ProvideOperation<O: Operation> {
  fn operation(&self, name: &String) -> Option<&O>;
}
