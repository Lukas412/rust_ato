pub trait Creation<Value: CreationValue<T>, T> {
  fn new(values: Vec<Value>) -> Self;
  fn value(&self, namespace: String, name: String) -> &Value;
}

pub trait CreationValue<T> {
  fn new(value: T, namespace: String, name: String) -> Self;
  fn value(&self) -> &T;
  fn namespace(&self) -> &String;
  fn name(&self) -> &String;
}