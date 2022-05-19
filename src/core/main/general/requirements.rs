use std::collections::HashMap;

use crate::core::main::general::creation::GeneralCreationOperation;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::namespace::{GetNamespace, Namespace};
use crate::core::traits::operation::ProvideOperation;
use crate::core::traits::pack::ProvidePack;
use crate::core::traits::requirements::RunWithRequirements;
use crate::{BuildableWithRequirements, GeneralPackProvider};

pub struct Requirements {
  pack_provider: &'static GeneralPackProvider,
  namespace: Namespace,
  stack: Vec<RequirementBox>,
}

impl Requirements {
  pub fn new(pack_provider: &'static GeneralPackProvider) -> Self {
    Self {
      pack_provider,
      namespace: Namespace::default(),
      stack: Vec::default(),
    }
  }
}

impl GetNamespace for Requirements {
  fn get_namespace(&self) -> &Namespace {
    match self.stack.last() {
      Some(last) => last.get_namespace(),
      None => &self.namespace,
    }
  }
}

impl ProvidePack<PathPack> for Requirements {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    self.pack_provider.path_packs()
  }
}

impl ProvidePack<StringPack> for Requirements {
  fn packs(&self) -> &HashMap<Namespace, StringPack> {
    self.pack_provider.string_packs()
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