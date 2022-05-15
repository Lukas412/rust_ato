use crate::core::main::general::operation::GeneralOperationProvider;
use crate::core::traits::namespace::{Namespace, GetNamespace};
use crate::GeneralPackProvider;

pub struct GeneralRequirements<'a> {
  namespace: &'a Namespace,
  operation_provider: &'a GeneralOperationProvider,
  pack_provider: &'a GeneralPackProvider,
}

impl GetNamespace for GeneralRequirements {
  fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}