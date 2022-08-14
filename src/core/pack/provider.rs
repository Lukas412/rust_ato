use std::collections::HashMap;
use std::path::Path;
use crate::core::namespace::Namespace;
use crate::core::pack::Pack;
use crate::errors::build::BuildError;
use crate::errors::build::pack::PackNotFoundError;

pub(crate) struct PackProvider {
  packs: HashMap<Namespace, Pack>,
}

impl PackProvider {
  pub(crate) fn from_root<P: AsRef<Path> + ?Sized>(root: &P) -> Self {
    let packs = Pack::all_from_root(root);
    Self { packs }
  }

  pub(crate) fn get_pack(&self, namespace: &Namespace) -> error_stack::Result<&Pack, PackNotFoundError> {
    match self.packs.get(namespace) {
      Some(pack) => Ok(pack),
      None => Err(PackNotFoundError::new_record(namespace.to_owned()))
    }
  }
}
