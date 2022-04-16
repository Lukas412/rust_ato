use std::str::FromStr;

pub trait Element<T>: FromStr {
  fn new(value: T) -> Self;
  fn value(&self) -> T;
}