use std::collections::HashMap;
use std::path::Path;
use crate::core::error::BuildError;
use crate::core::namespace::Namespace;
use crate::core::pack::Pack;

pub struct PackProvider {
  packs: HashMap<Namespace, Pack>,
}

impl PackProvider {
  pub fn from_root<P: AsRef<Path> + ?Sized>(root: &P) -> Self {
    let packs = Pack::from_root(root);
    Self { packs }
  }

  pub fn get_pack(&self, namespace: &Namespace) -> Result<&Pack, BuildError> {
    match self.packs.get(namespace) {
      Some(pack) => Ok(pack),
      None => Err(BuildError::new_pack_not_found_error(namespace.to_owned()))
    }
  }
}
