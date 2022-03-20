pub mod boolean;
pub mod number;
pub mod path;
pub mod string;

pub trait Element<T> {
  fn new(value: T) -> Self;
  fn get_value(&self) -> T;
}