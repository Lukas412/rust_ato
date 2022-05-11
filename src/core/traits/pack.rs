use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;
use std::iter::{FilterMap, FromIterator, Map};
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, FilterEntry, IntoIter, WalkDir};
use yaserde::de::from_str;
use yaserde::YaDeserialize;

use crate::{Buildable, Container};
use crate::core::main::namespace::Namespace;
use crate::core::main::path::pack::PathPack;

pub trait Pack<B, E, C>: Buildable<B, E, C>
  where
    Self: Debug + YaDeserialize,
    C: Container
{
  const SUFFIX: &'static str;

  fn namespace(&self) -> &String;

  fn build_with_requirements<const N: usize>(&self, elements: [(String, C::Value); N]) -> Result<B, E> {
    let namespace = self.namespace().to_owned();
    let requirements = C::from(namespace, elements);
    self.build(&requirements)
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