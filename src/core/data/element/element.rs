pub trait Element<T> {
  fn new(value: T) -> Self;
  fn value(&self) -> T;
}