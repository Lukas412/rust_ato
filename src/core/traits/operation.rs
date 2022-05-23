pub trait Operation {
  type Value;
}

pub trait ProvideOperation<O: Operation> {
  fn operation(&self, name: &String) -> Option<&O>;
}

pub trait ProvideOperationWithNamespace<O: Operation> {
  fn operation(&self, namespace: &String, name: &String) -> Option<&O>;
}

pub trait GetOperation<O>
  where O: Operation
{
  fn get_operation(&self) -> O;
}