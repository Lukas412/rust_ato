#[derive(Debug, Default)]
pub struct Arguments(Vec<Argument>);

impl Arguments {
  pub fn new(arguments: Vec<Argument>) -> Self {
    Self(arguments)
  }
}

#[derive(Debug)]
pub enum Argument {
  Boolean(),
  Number(),
  Path(),
  String(),
}