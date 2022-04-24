pub trait Argument<T> {
  fn value(&self) -> T;
}