use crate::BuildError;

pub trait Operation {
  type Value;
}

pub trait ProvideOperation<O>
  where O: Operation
{
  fn operation(&self, name: &String) -> Result<O, BuildError>;
}

pub trait ProvideOperationWithNamespace<O>
  where O: Operation
{
  fn operation(&self, namespace: &String, name: &String) -> Result<O, BuildError>;
}

pub trait GetOperation<O>
  where O: Operation
{
  fn get_operation(&self) -> O;
}