use crate::core::traits::element::Element;
use crate::core::data::requirement::Requirements;
use crate::core::traits::build::BuildableWithRequirements;
use crate::core::traits::file::File;

pub trait Pack<E: Element<T>, T>: File + BuildableWithRequirements<E, String, Requirements> {}