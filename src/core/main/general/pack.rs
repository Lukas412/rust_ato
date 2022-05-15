use std::collections::HashMap;
use std::path::Path;

use crate::core::traits::namespace::Namespace;
use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::pack::{Pack, ProvidePack};

pub struct GeneralPackProvider {
  path_packs: HashMap<Namespace, PathPack>,
  string_packs: HashMap<Namespace, StringPack>,
}

impl GeneralPackProvider {
  pub fn from_root<P: AsRef<Path> + ?Sized>(root: &P) -> Self {
    Self {
      path_packs: PathPack::from_root(root),
      string_packs: StringPack::from_root(root),
    }
  }
}

impl ProvidePack<PathPack> for GeneralPackProvider {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    &self.path_packs
  }
}

impl ProvidePack<StringPack> for GeneralPackProvider {
  fn packs(&self) -> &HashMap<Namespace, StringPack> {
    &self.string_packs
  }
}