use crate::core::traits::container::Container;

pub struct ElementContainer {

}

impl Container for ElementContainer {
  fn value<T>(&self, namespace: String, name: String) -> T {
    todo!()
  }
}