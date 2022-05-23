pub trait Operation {
  type Value;
}

pub trait ProvideOperation<O: Operation> {
  fn operation(&self, namespace: &String, name: &String) -> Option<&O>;
}

pub trait ToOperation<O>
  where O: Operation
{
  fn to_operation(self) -> O;
}