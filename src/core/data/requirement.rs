pub struct Requirements(Vec<Requirement>);

impl Requirements {
  pub fn empty() -> Self {
    Self(vec![])
  }

  pub fn new(requirements: Vec<Requirement>) -> Self {
    Self(requirements)
  }
}

pub struct Requirement;