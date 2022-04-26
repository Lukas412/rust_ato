use crate::core::traits::container::Container;

#[derive(Debug, Default)]
pub struct ElementContainer {

}

impl Container for ElementContainer {
  fn value<T>(&self, namespace: String, name: String) -> T {
    todo!()
  }
}