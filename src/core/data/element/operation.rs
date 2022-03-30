use crate::core::concepts::build::BuildableWithRequirements;
use crate::core::data::element::element::Element;
use crate::core::data::requirement::Requirements;

pub trait Operation<E: Element<T>, T>: BuildableWithRequirements<E, Requirements> {}