use crate::concepts::Builder;

pub mod boolean;
pub mod number;
pub mod path;
pub mod string;

pub trait Element<T> {
  fn new(value: T) -> Self;
  fn value(&self) -> T;
}

pub trait Operation<E: Element<T>, T>: Builder<Self, E> {}

pub trait Parameter {}
