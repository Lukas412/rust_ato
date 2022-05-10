use std::fs;
use std::path::{Path, PathBuf};

use crate::core::build::error::BuildError;
use crate::core::traits::file::File;
use crate::GeneralBundle;

pub struct PackProvider {}

impl PackProvider {
  fn from_bundles(paths: Vec<&Path>) -> Result<Self, BuildError> {
    let mut bundles = Vec::new();
    for path in paths {
      match GeneralBundle::from_file(path) {
        Ok(bundle) => bundles.push(bundle),
        Err(error) => Err(BuildError::new_init(error)),
      }
    }
    Ok(Self)
  }
}