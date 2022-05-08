use crate::{Buildable, Container};

pub trait Pack<Build, Error, C: Container>: Buildable<Build, Error, C> {
  fn namespace(&self) -> &String;

  fn build_with_requirements<const N: usize>(&self, elements: [(String, C::Value); N]) -> Result<Build, Error> {
    let namespace = self.namespace().to_owned();
    let requirements = C::from(namespace, elements);
    self.build(&requirements)
  }
}