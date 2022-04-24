use crate::core::traits::argument::Argument;

#[derive(Debug, Default)]
pub struct ElementArguments {
  arguments: Vec<ElementArgument>,
}

#[derive(Debug)]
pub enum ElementArgument {
  Boolean(),
  Number(),
  Path(),
  String(),
}

impl<T> Argument<T> for ElementArgument {
  fn value(&self) -> T {
    todo!()
  }
}