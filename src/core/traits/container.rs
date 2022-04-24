pub trait Container {
  fn value<T>(&self, namespace: String, name: String) -> T;
}