use std::collections::HashMap;
use std::path::Path;

use crate::core::main::namespace::Namespace;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::pack::Pack;

pub struct PackProvider {
  path_packs: HashMap<Namespace, PathPack>,
  string_packs: HashMap<Namespace, StringPack>,
}

impl PackProvider {
  pub(crate) fn from_root(root: &Path) -> Self {
    Self {
      path_packs: PathPack::from_root(root),
      string_packs: StringPack::from_root(root),
    }
  }

  pub fn path_pack(&self, namespace: &Namespace) -> Option<&PathPack> {
    self.path_packs.get(namespace)
  }

  pub fn string_pack(&self, namespace: &Namespace) -> Option<&StringPack> {
    self.string_packs.get(namespace)
  }
}