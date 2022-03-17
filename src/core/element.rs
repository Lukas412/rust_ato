mod boolean;
mod number;
mod path;
mod string;

trait Element<T> {
  fn new(value: T) -> &Self;
  fn get_value(&self) -> T;
}