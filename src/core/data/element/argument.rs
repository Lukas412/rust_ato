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