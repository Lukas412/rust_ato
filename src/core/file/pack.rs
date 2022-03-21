use std::collections::hash_set::Union;
use crate::BooleanParameter;
use crate::core::element::{Element, Parameter};
use crate::core::file::File;

struct Pack {
  namespace: String,
  parameters: Option<Vec<dyn Parameter>>,
  element: dyn Element<T>,
}