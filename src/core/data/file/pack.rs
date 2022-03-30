use crate::concepts::BuildableWithRequirements;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;
use crate::core::data::file::File;
use crate::core::data::requirement::Requirements;

pub trait Pack<O: Operation<E, T>, E: Element<T>, T>: BuildableWithRequirements<O, E, Requirements> + File {}