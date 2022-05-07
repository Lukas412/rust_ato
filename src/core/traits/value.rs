pub trait Value {
  type Type;

  fn new(value: Self::Type, namespace: String) -> Self;
  fn value(&self) -> &Self::Type;
  fn namespace(&self) -> &String;
}

pub trait ValueFromString: Value {
  fn from_string(string: &str, namespace: &String) -> Box<Self> {
    let value = Self::value_from_string(string);
    Self::new(value, namespace.to_owned())
  }

  fn value_from_string(string: &str) -> Self::Type;
}