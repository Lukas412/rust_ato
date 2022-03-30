use crate::core::concepts::build::Buildable;
use crate::core::data::element::element::Element;

pub trait Operation<E: Element<T>, T>: Buildable<E> {}