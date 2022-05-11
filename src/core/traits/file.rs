use std::{fs, io};
use std::path::{Path, PathBuf};

use crate::core::main::namespace::{Namespace, path_from_namespace};

pub trait File
  where Self: Sized
{
  const SUFFIX: &'static str;

  fn read_file(root: &Path, namespace: &Namespace) -> io::Result<String> {
    let path = concreate_path_from_namespace(root, namespace, Self::SUFFIX);
    fs::read_to_string(path.with_extension(Self::SUFFIX))
  }
}

fn concreate_path_from_namespace(root: &Path, namespace: &Namespace, suffix: &str) -> PathBuf {
  let namespace_path = path_from_namespace(namespace);
  root.join(namespace_path).with_extension(suffix)
}