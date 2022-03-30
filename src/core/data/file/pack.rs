use crate::concepts::BuilderWithRequirements;
use crate::core::data::element::{Element, Operation};
use crate::core::data::file::File;
use crate::core::data::requirement::Requirements;

pub trait Pack<O: Operation<E, T>, E: Element<T>, T>: BuilderWithRequirements<O, E, Requirements> + File {}