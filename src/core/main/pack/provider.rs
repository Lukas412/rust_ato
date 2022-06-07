use std::collections::HashMap;
use std::path::Path;
use crate::core::main::general::pack::Pack;

use crate::core::main::namespace::Namespace;

pub struct PackProvider {
  packs: HashMap<Namespace, Pack>,
}

impl PackProvider {
  pub fn from_root<P: AsRef<Path> + ?Sized>(root: &P) -> Self {
    let packs = Pack::from_root(root);
    Self { packs }
  }

  pub fn get_pack(&self, namespace: &Namespace) -> &Pack {
    &self.packs[namespace]
  }
}
