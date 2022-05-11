use std::{fs, io};
use std::path::{Path, PathBuf};

use yaserde::de::from_str;
use yaserde::YaDeserialize;

pub trait File
  where Self: Sized + YaDeserialize
{
  fn suffix() -> String;

  fn read_file(path: &Path) -> Option<String> {
    fs::read_to_string(path.with_extension(Self::suffix())).ok()
  }
}