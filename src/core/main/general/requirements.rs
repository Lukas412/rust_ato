use std::collections::HashMap;
use yaserde::__xml::name::Name;
use crate::core::main::general::creation::GeneralCreationValue;
use crate::core::main::general::operation::GeneralOperationProvider;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::namespace::{Namespace, GetNamespace};
use crate::core::traits::pack::ProvidePack;
use crate::GeneralPackProvider;

pub struct GeneralRequirements {
  pack_provider: &'static GeneralPackProvider,
  namespace: Namespace,
  operation_provider: GeneralOperationProvider,
}

impl GeneralRequirements {
  pub fn next(&self, namespace: Namespace, values: &Vec<GeneralCreationValue>) -> Self {
    let operation_provider = self.next_operation_provider(values);
    Self {
      pack_provider: self.pack_provider,
      namespace,
      operation_provider,
    }
  }
}

impl GeneralRequirements {
  pub fn new(pack_provider: &'static GeneralPackProvider) -> Self {
    let namespace = Namespace::default();
    let operation_provider = GeneralOperationProvider::default();
    Self {
      pack_provider,
      namespace,
      operation_provider,
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