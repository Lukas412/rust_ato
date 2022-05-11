use std::collections::HashMap;
use std::path::Path;

use crate::core::build::error::BuildError;
use crate::core::main::namespace::Namespace;
use crate::core::main::path::pack::PathPack;
use crate::core::traits::file::File;
use crate::{GeneralBundle, StringPack};

pub struct PackProvider {
  path_packs: HashMap<Namespace, PathPack>,
  string_packs: HashMap<Namespace, StringPack>,
}

impl PackProvider {
  fn from_root(root: &Path) -> Result<Self, BuildError> {
    Ok(Self)
  }

  fn path_pack(&self, namespace: &Namespace) -> Option<&PathPack> {
    self.path_packs.get(namespace)
  }

  fn string_pack(&self, namespace: &Namespace) -> Option<&StringPack> {
    self.string_packs.get(namespace)
  }
}