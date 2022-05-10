use std::fs;
use std::path::Path;

use crate::core::build::error::BuildError;

pub struct PackProvider {}

impl PackProvider {
  fn load_from_location(location: &Path) -> Result<Self, BuildError> {
    let paths_result = fs::read_dir(location);
    match paths_result {
      Ok(paths) => {
        for path_result in paths {
          let path = match path_result {
            Ok(path) => path.path(),
            Err(_) => continue,
          };
        }
        Ok(Self {})
      }
      Err(error) => Err(BuildError::new_init(format!("error while loading modules: {}", error))),
    }
  }
}