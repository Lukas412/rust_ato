use std::path::Path;

use crate::core::build::error::BuildError;
use crate::core::traits::file::File;
use crate::GeneralBundle;

pub struct PackProvider {}

impl PackProvider {
  fn from_bundle_paths(paths: Vec<Path>) -> Result<Self, BuildError> {
    let bundles = paths.iter()
      .filter_map(GeneralBundle::from_file)
      .collect();
    Ok(Self)
  }
}