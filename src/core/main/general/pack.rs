use crate::core::main::general::operation::GeneralOperation;
use crate::core::main::general::parameter::GeneralParameters;
use crate::core::traits::namespace::Namespace;

pub mod provider;

struct Pack {
  namespace: Namespace,
  parameters: GeneralParameters,
  operation: GeneralOperation,
}
