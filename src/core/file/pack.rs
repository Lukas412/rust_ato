use std::any::Any;
use crate::core::element::{Element, Operation, Parameter};
use crate::core::file::File;

trait Pack<O: Operation<E, dyn Any>, E: Element<dyn Any>> {}