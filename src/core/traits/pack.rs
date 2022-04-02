use crate::core::traits::element::Element;
use crate::core::traits::operation::Operation;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;

pub trait Pack<O: Operation<E, T>, E: Element<T>, T>: File + BuildableWithRequirements<E, Requirements> {}