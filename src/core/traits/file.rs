use std::fs;
use std::path::Path;

use yaserde::de::from_str;
use yaserde::YaDeserialize;

pub trait File
  where Self: Sized + YaDeserialize
{
  fn suffix() -> String;
  fn from_file(path: &Path) -> Result<Self, String> {
    let suffix = Self::suffix();
    if !path.ends_with(&suffix) {
      return Err(format!("filename needs to end with: {}", &suffix));
    };
    match fs::read_to_string("src/bundles/angular/cli/init/name.string.xml") {
      Ok(string) => from_str(&string),
      Err(error) => Err(format!("{}", error)),
    }
  }
}