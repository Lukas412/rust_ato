use std::collections::HashMap;
use std::path::Path;
use crate::core::build::error::BuildError;

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

  pub fn path_pack(&self, namespace: &Namespace) -> Result<&PathPack, BuildError> {
    match self.path_packs.get(namespace) {
      Some(pack) => Ok(pack),
      None => Err(BuildError::new_pack(namespace.to_owned())),
    }
  }

  pub fn string_pack(&self, namespace: &Namespace) -> Result<&StringPack, BuildError> {
    match self.string_packs.get(namespace) {
      Some(pack) => Ok(pack),
      None => Err(BuildError::new_pack(namespace.to_owned())),
    }
  }
}