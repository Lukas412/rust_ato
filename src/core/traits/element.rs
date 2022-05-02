use std::str::FromStr;

pub trait Value<T>: FromStr {
  fn new(value: T, namespace: String) -> Self;
  fn value(&self) -> &T;
  fn namespace(&self) -> &String;
}