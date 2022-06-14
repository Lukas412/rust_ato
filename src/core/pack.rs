use std::collections::HashMap;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

use crate::common::ends_with::EndsWithStr;
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
        .filter(is_pack_path)
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
}

fn is_pack_path(path: &PathBuf) -> bool {
  let extensions = [".action.xml", ".boolean.xml", ".number.xml", ".path.xml", ".string.xml"];
  extensions.iter().any(|extension| path.ends_with_str(extension))
}