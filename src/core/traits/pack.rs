use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::iter::FromIterator;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};
use yaserde::de::from_str;
use yaserde::YaDeserialize;

use crate::core::main::namespace::Namespace;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Container;

pub trait Pack
  where Self: Debug + YaDeserialize
{
  const SUFFIX: &'static str;

  fn namespace(&self) -> &String;

  fn requirements<C: Container, const N: usize>(&self, elements: [(String, C::Value); N]) -> C {
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
        .map(read_to_string)
        .filter_map(Result::ok)
        .map(|s: String| from_str::<Self>(&s))
        .filter_map(Result::ok)
        .map(|pack: Self| (pack.namespace().to_owned(), pack));
    HashMap::from_iter(packs)
  }

  fn is_pack_path(path: &PathBuf) -> bool {
    path.to_str().unwrap().ends_with(Self::SUFFIX)
  }
}