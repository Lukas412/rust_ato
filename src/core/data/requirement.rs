#[derive(Debug, Default)]
pub struct Requirements(Vec<Requirement>);

impl Requirements {
  pub fn new(requirements: Vec<Requirement>) -> Self {
    Self(requirements)
  }
}

#[derive(Debug)]
pub struct Requirement;