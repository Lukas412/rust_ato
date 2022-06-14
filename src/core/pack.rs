use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};
use walkdir::{WalkDir, DirEntry};
use crate::core::namespace::Namespace;
use crate::core::operation::Operation;
use crate::core::parameter::Parameters;
use crate::from_file;

pub mod provider;

struct Pack {
  namespace: Namespace,
  parameters: Parameters,
  operation: Operation,
}

impl Pack {
  pub fn all_from_root<P: AsRef<Path> + ?Sized>(root: &P) -> HashMap<Namespace, Self> {
    let packs =
      WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .map(DirEntry::into_path)
        .filter(Self::is_pack_path)
        .map(from_file)
        .filter_map(Result::ok)
        .map(|pack: Self| (pack.get_owned_namespace(), pack));
    HashMap::from_iter(packs)
  }

  pub fn get_operation(&self) -> &Operation {
    &self.operation
  }
}

impl Pack {
  fn get_owned_namespace(&self) -> Namespace {
    self.namespace.to_owned()
  }

  fn is_pack_path(path: &PathBuf) -> bool {
    let extensions = [".action.xml", ".boolean.xml", ".number.xml", ".path.xml", ".string.xml"];
    extensions.iter().map(path.ends_with_str).any()
  }
}