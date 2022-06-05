use std::collections::HashMap;
use std::path::Path;
use crate::core::main::path::pack::PathPack;
use crate::core::main::path::value::PathValue;
use crate::core::traits::namespace::Namespace;
use crate::core::traits::pack::{Pack, ProvidePack};

pub struct PackProvider {
  packs: HashMap<Namespace, PathPack>,
}

impl PackProvider {
  pub fn from_root<P: AsRef<Path> + ?Sized>(root: &P) -> Self {
    Self {
      packs: PathPack::from_root(root),
    }
  }
}

impl ProvidePack<PathPack, PathValue> for PackProvider {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    &self.packs
  }
}
