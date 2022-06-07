use crate::core::main::general::operation::Operation;
use crate::core::main::general::parameter::Parameters;
use crate::core::main::namespace::Namespace;

pub mod provider;

struct Pack {
  namespace: Namespace,
  parameters: Parameters,
  operation: Operation,
}
