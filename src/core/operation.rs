pub trait Operation<T> {
  fn build(&self) -> T;
}