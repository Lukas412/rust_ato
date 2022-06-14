use std::collections::HashMap;
use std::fs::DirEntry;
use std::iter::FromIterator;
use std::path::Path;
use walkdir::WalkDir;
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
  pub fn all_from_root<P: AsRef<Path>>(root: &P) -> HashMap<Namespace, Self> {
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

  pub fn get_operation(&self) -> &Operation {
    &self.operation
  }
}

impl Pack {
  fn is_pack_path<P: AsRef<Path>>(path: &P) -> bool {
    todo!()
  }
}