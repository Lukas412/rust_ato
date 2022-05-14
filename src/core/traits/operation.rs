pub trait Operation {
  type Type;
}

pub trait ProvideOperation<O: Operation> {
  fn operation(&self, name: &String) -> Option<&O>;
}
