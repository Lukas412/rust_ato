use crate::core::concepts::build::BuildableWithRequirements;
use crate::core::data::element::element::Element;
use crate::core::data::element::operation::Operation;
use crate::core::data::file::File;
use crate::core::data::requirement::Requirements;

pub trait Pack<O: Operation<E, T>, E: Element<T>, T>: BuildableWithRequirements<E, Requirements> + File {}