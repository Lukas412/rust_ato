use std::collections::HashMap;
use std::path::Path;
use crate::BuildError;

use crate::core::main::path::pack::PathPack;
use crate::core::main::string::pack::StringPack;
use crate::core::traits::namespace::Namespace;
use crate::core::traits::operation::{Operation, ProvideOperation};
use crate::core::traits::pack::{Pack, ProvidePack};

pub struct PackProvider {
  path_packs: HashMap<Namespace, PathPack>,
  string_packs: HashMap<Namespace, StringPack>,
}

impl PackProvider {
  pub fn from_root<P: AsRef<Path> + ?Sized>(root: &P) -> Self {
    Self {
      path_packs: PathPack::from_root(root),
      string_packs: StringPack::from_root(root),
    }
  }
}

impl ProvidePack<PathPack> for PackProvider {
  fn packs(&self) -> &HashMap<Namespace, PathPack> {
    &self.path_packs
  }
}

impl ProvidePack<StringPack> for PackProvider {
  fn packs(&self) -> &HashMap<Namespace, StringPack> {
    &self.string_packs
  }
}

impl<O> ProvideOperation<O> for PackProvider
  where O: Operation
{
  fn operation(&self, name: &String) -> Result<O, BuildError> {
    todo!()
  }
}