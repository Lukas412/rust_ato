use crate::Container;

pub trait Pack<C: Container> {
  fn requirements(&self) -> C;
}