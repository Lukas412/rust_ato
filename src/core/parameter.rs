use std::io::Read;

use serde::Deserialize;

use crate::core::namespace::Namespace;
use crate::core::parameter::inner::InnerParameter;
use crate::core::variant::Variant;

pub(crate) mod inner;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct Parameters {
  parameters: Vec<Parameter>,
}

#[derive(Debug)]
pub(crate) struct Parameter {
  name: String,
  namespace: Option<Namespace>,
  variant: Variant,
}
