use crate::core::namespace::Namespace;
use crate::core::operation::Operation;
use crate::core::parameter::Parameters;

pub mod provider;

struct Pack {
  namespace: Namespace,
  parameters: Parameters,
  operation: Operation,
}

impl Pack {
  pub fn get_operation(&self) -> &Operation {
    &self.operation
  }
}