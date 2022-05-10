use crate::{Buildable, Container};
use crate::core::traits::file::File;

pub trait Pack<B, E, C>: File + Buildable<B, E, C>
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

pub trait PackProvider<P, B, E, C>
where
  P: Pack<B, E, C>,
  C: Container
{
  fn get_pack(&self) -> P;
}