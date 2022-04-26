pub trait Container<Value> {
  fn new(values: Vec<Value>) -> Self;
  fn value(&self, namespace: String, name: String) -> Value;
}