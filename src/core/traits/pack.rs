use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};
use yaserde::YaDeserialize;

use crate::core::build::error::BuildError;
use crate::core::main::namespace::Namespace;
use crate::from_file;

pub trait Pack: Debug + YaDeserialize
{
  const SUFFIX: &'static str;

  fn namespace(&self) -> &Namespace;

  fn from_root<P: AsRef<Path>>(root: P) -> HashMap<Namespace, Self> {
    let packs =
      WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .map(DirEntry::into_path)
        .filter(Self::is_pack_path)
        .map(from_file)
        .filter_map(Result::ok)
        .map(|pack: Self| (pack.namespace().to_owned(), pack));
    HashMap::from_iter(packs)
  }

  fn is_pack_path(path: &PathBuf) -> bool {
    path.to_str().unwrap().ends_with(Self::SUFFIX)
  }
}

pub trait ProvidePack<P>
  where P: Pack
{
  fn packs(&self) -> &HashMap<Namespace, P>;

  fn pack(&self, namespace: &Namespace) -> Result<&P, BuildError> {
    match self.packs().get(namespace) {
      Some(pack) => Ok(pack),
      None => Err(BuildError::new_pack(namespace.to_owned())),
    }
  }
}