use crate::core::element::Parameter;

pub struct NumberParameter {
  name: String,
  namespace: Option<String>,
}

impl Parameter for NumberParameter {}