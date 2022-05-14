use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};
use yaserde::de::from_str;
use yaserde::YaDeserialize;

use crate::core::main::namespace::Namespace;
use crate::core::traits::container::Container;
use crate::from_file;

pub trait Pack
  where Self: Debug + YaDeserialize
{
  const SUFFIX: &'static str;

  fn namespace(&self) -> &String;

  fn requirements<C: Container>(&self, elements: Vec<(String, C::Value)>) -> C {
    let namespace = self.namespace().to_owned();
    C::from(namespace, elements)
  }

  fn from_root(root: &Path) -> HashMap<Namespace, Self> {
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