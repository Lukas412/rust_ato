use crate::{Buildable, Container};

pub trait Pack<B, E, C>: Buildable<B, E, C>
  where
    C: Container
{
  fn namespace(&self) -> &String;

  fn build_with_requirements<const N: usize>(&self, elements: [(String, C::Value); N]) -> Result<B, E> {
    let namespace = self.namespace().to_owned();
    let requirements = C::from(namespace, elements);
    self.build(&requirements)
  }
}