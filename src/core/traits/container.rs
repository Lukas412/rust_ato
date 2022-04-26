pub trait Container<Value: ContainerValue<T>, T> {
  fn new(values: Vec<Value>) -> Self;
  fn value(&self, namespace: String, name: String) -> Value;
}

pub trait ContainerValue<T> {
  fn value(&self) -> T;
  fn name(&self) -> String;
  fn namespace(&self) -> String;
}