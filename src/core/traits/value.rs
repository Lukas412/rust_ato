pub trait Value {
  type Type;

  fn new(value: Self::Type, namespace: String) -> Self;
  fn value(&self) -> &Self::Type;
  fn namespace(&self) -> &String;
}