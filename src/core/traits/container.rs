use crate::core::traits::element::Element;

pub trait Container<E: Element<T>, T> {
  fn element(&self) -> &E;
  fn name(&self) -> &String;
  fn namespace(&self) -> &String;
}