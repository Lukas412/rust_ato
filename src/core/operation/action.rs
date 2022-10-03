use std::io::Read;
use std::rc::Rc;

use serde::Deserialize;

use crate::core::namespace::ParameterName;
use crate::Creation;

#[derive(Debug)]
pub(crate) enum OperationAction {
  Empty,
  Creation(Rc<Creation>),
  Value(String),
  GetArgument(ParameterName),
}

impl OperationAction {
  pub(crate) fn new_creation(creation: Rc<Creation>) -> Self {
    Self::Creation(creation)
  }

  pub(crate) fn new_empty() -> Self {
    Self::Empty
  }

  pub(crate) fn new_value(value: String) -> Self {
    Self::Value(value)
  }

  pub(crate) fn new_get_argument(name: ParameterName) -> Self {
    Self::GetArgument(name)
  }
}

impl Default for OperationAction {
  fn default() -> Self {
    Self::Empty
  }
}

#[derive(Debug, Deserialize)]
struct InnerValueOperationAction {
  text: String,
}

#[derive(Debug, Deserialize)]
struct InnerGetArgumentOperationAction {
  namespace: String,
  name: String,
}

impl InnerGetArgumentOperationAction {
  fn to_parameter_name(self) -> ParameterName {
    ParameterName::new(self.namespace, self.name)
  }
}