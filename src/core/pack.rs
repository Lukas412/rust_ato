use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};
use helpers::ser::from::from_file;

use crate::helpers::ends_with::EndsWithStr;
use crate::core::namespace::Namespace;
use crate::core::operation::Operation;
use crate::core::parameter::Parameters;

pub(crate) mod provider;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(rename = "pack")]
pub(crate) struct Pack {
  #[yaserde(attribute)]
  namespace: Namespace,
  parameters: Parameters,
  operation: Operation,
}

impl Pack {
  pub(crate) fn all_from_root<P: AsRef<Path> + ?Sized>(root: &P) -> HashMap<Namespace, Self> {
    let packs =
      WalkDir::new(root).into_iter().filter_map(Pack::namespace_and_pack_from_dir_entry);
    HashMap::from_iter(packs)
  }

  pub(crate) fn get_operation(&self) -> &Operation {
    &self.operation
  }
}

impl Pack {
  fn namespace_and_pack_from_dir_entry(dir_entry: walkdir::Result<DirEntry>) -> Option<(Namespace, Pack)> {
    let path = dir_entry.ok()?.into_path();
    if !is_pack_path(&path) {
      return None;
    }
    let pack: Self = from_file(&path).ok()?;
    Some(pack.get_namespace_and_pack())
  }

  fn get_namespace_and_pack(self) -> (Namespace, Pack) {
    (self.get_owned_namespace(), self)
  }

  fn get_owned_namespace(&self) -> Namespace {
    self.namespace.to_owned()
  }
}

fn is_pack_path(path: &PathBuf) -> bool {
  let extensions = [".action.xml", ".boolean.xml", ".number.xml", ".path.xml", ".string.xml"];
  extensions.iter().any(|extension| path.ends_with_str(extension))
}