use std::fs;
use std::path::{Path, PathBuf};

use crate::core::build::error::BuildError;
use crate::core::traits::file::File;
use crate::GeneralBundle;

pub struct PackProvider {}

impl PackProvider {
  fn from_bundles(paths: Vec<Path>) -> Result<Self, BuildError> {
    let bundles = paths.iter()
      .filter_map(GeneralBundle::from_file);
    Ok(Self)
  }
}