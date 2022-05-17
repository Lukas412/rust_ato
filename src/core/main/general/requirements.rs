use std::collections::HashMap;

use crate::core::main::general::creation::GeneralCreationOperation;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::namespace::{GetNamespace, Namespace};
use crate::core::traits::pack::ProvidePack;
use crate::GeneralPackProvider;

pub struct Requirements {
  pack_provider: &'static GeneralPackProvider,
  stack: Vec<RequirementBox>,
}

impl Requirements {
  pub fn new(pack_provider: &'static GeneralPackProvider) -> Self {
    Self {
      pack_provider,
      stack: Vec::default()
    }
  }
}

pub struct RequirementBox {
  namespace: Namespace,
  operations: HashMap<String, GeneralCreationOperation>,
}

impl RequirementBox {
  pub fn new(namespace: Namespace, operations: HashMap<String, GeneralCreationOperation>) -> Self {
    Self {
      namespace,
      operations,
    }
  }
}

impl GetNamespace for RequirementBox {
  fn get_namespace(&self) -> &Namespace {
    &self.namespace
  }
}

impl ProvidePack<PathPack> for RequirementBox {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    self.pack_provider.path_packs()
  }
}

impl ProvidePack<StringPack> for RequirementBox {
  fn packs(&self) -> &HashMap<Namespace, StringPack> {
    self.pack_provider.string_packs()
  }
}