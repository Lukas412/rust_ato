use std::collections::hash_set::Union;
use crate::BooleanParameter;
use crate::core::element::{Element, Parameter};
use crate::core::file::File;

struct Pack<E, P: Parameter> {
  namespace: String,
  parameters: Option<Vec<P>>,
  element: dyn Element<E>,
}