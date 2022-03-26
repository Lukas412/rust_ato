use crate::core::element::{Element, Operation};

pub trait Pack<O: Operation<E, T>, E: Element<T>, T> {}