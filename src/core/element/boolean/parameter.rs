use crate::core::element::Parameter;

pub struct BooleanParameter {
  name: String,
  namespace: Option<String>,
}

impl Parameter for BooleanParameter {}