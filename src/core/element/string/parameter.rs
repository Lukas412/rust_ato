use crate::core::element::Parameter;

pub struct StringParameter {
  name: String,
  namespace: Option<String>,
}

impl Parameter for StringParameter {}