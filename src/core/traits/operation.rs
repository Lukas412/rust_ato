use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::element::Element;
use crate::core::data::requirement::Requirements;

pub trait Operation<E: Element<T>, T>: BuildableWithRequirements<E, String, Requirements> {}