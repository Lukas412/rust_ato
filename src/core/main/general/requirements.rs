use std::collections::HashMap;
use crate::core::main::general::creation::GeneralCreationValue;
use crate::core::main::general::operation::GeneralOperationProvider;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::namespace::{Namespace, GetNamespace};
use crate::core::traits::pack::ProvidePack;
use crate::GeneralPackProvider;

pub struct GeneralRequirements {
  pack_provider: &'static GeneralPackProvider,
  stack: Vec<RequirementBox>
}

pub struct RequirementBox {
  namespace: Namespace,
  operation_provider: GeneralOperationProvider,
}

impl RequirementBox {
  pub fn new(namespace: Namespace, values: Vec<GeneralCreationValue>) -> Self {
    Self {
      namespace,
      operation_provider: GeneralOperationProvider::new(values),
    }
  }
}

impl GetNamespace for GeneralRequirements {
  fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}

impl ProvidePack<PathPack> for GeneralRequirements {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    self.pack_provider.path_packs()
  }
}

impl ProvidePack<StringPack> for GeneralRequirements {
  fn packs(&self) -> &HashMap<Namespace, StringPack> {
    self.pack_provider.string_packs()
  }
}