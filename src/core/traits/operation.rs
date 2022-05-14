pub trait Operation {
  type Type;
}

pub trait ProvideOperation<O: Operation> {
  fn operation(&self) -> O;
}
