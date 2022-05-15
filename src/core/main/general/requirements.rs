use std::collections::HashMap;
use crate::core::main::general::operation::GeneralOperationProvider;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::namespace::{Namespace, GetNamespace};
use crate::core::traits::pack::ProvidePack;
use crate::GeneralPackProvider;

pub struct GeneralRequirements<'a> {
  namespace: &'a Namespace,
  pack_provider: &'a GeneralPackProvider,
  operation_provider: GeneralOperationProvider,
}

impl GeneralRequirements<'_> {
  const DEFAULT_NAMESPACE: Namespace = "__namespace__".to_owned();

  pub fn new(pack_provider: &'static GeneralPackProvider) -> Self {
    let namespace = &Self::DEFAULT_NAMESPACE;
    let operation_provider = GeneralOperationProvider::default();
    Self {
      namespace,
      pack_provider,
      operation_provider,
    }
  }
}

impl GetNamespace for GeneralRequirements<'_> {
  fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}

impl ProvidePack<PathPack> for GeneralRequirements<'_> {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    &self.pack_provider.path_packs()
  }
}

impl ProvidePack<StringPack> for GeneralRequirements<'_> {
  fn packs(&self) -> &HashMap<Namespace, StringPack> {
    &self.pack_provider.string_packs()
  }
}