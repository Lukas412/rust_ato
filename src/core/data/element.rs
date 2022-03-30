use crate::concepts::Buildable;

pub mod boolean;
pub mod number;
pub mod path;
pub mod string;

pub trait Element<T> {
  fn new(value: T) -> Self;
  fn value(&self) -> T;
}

pub trait Operation<E: Element<T>, T>: Buildable<E> {}

pub trait Parameter {}
