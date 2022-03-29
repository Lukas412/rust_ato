use crate::concepts::BuilderWithRequirements;
use crate::core::element::{Element, Operation};
use crate::core::file::File;
use crate::core::requirement::Requirements;

pub trait Pack<O: Operation<E, T>, E: Element<T>, T>: BuilderWithRequirements<O, E, Requirements> + File {}