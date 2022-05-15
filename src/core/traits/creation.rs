use crate::core::traits::operation::Operation;

pub trait CreationValue<O>
  where O: Operation
{
  fn to_operation(self) -> O;
}