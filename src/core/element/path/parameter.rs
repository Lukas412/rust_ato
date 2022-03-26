use crate::core::element::Parameter;

pub struct PathParameter {
  name: String,
  namespace: Option<String>,
}

impl Parameter for PathParameter {}