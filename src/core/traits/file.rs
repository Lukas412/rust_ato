use std::{fs, io};
use std::path::Path;

use yaserde::de::from_str;
use yaserde::YaDeserialize;

pub trait File
  where Self: Sized + YaDeserialize
{
  fn suffix() -> String;
  fn from_file(path: &Path) -> Option<Self> {
    if path.ends_with(Self::suffix()) {
      fs::read_to_string(path).ok()
    } else {
      None
    }
  }
}