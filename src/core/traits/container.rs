pub trait Container<T> {
  fn value(&self, namespace: String, name: String) -> T;
}