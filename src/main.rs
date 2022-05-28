extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::path::Path;
use crate::core::build::error::BuildError;
use crate::core::main::general::pack::PackProvider;
use crate::core::main::general::requirements::Requirements;
use crate::core::parse::from_file;
use crate::core::traits::build::BuildableWithRequirements;


use self::core::main::general::creation::GeneralCreation;

mod core;

fn main() {
  let pack_provider = PackProvider::from_root("src/bundles");
  build(&pack_provider, "src/creations/test2.creation.xml");
}

fn build<P: AsRef<Path>>(pack_provider: &PackProvider, file: P) {
  let mut requirements = Requirements::new();
  let creation: GeneralCreation = from_file(file).unwrap();
  println!("{:?}", creation);

  let value = creation.build(&pack_provider, &mut requirements);
  match value {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error)
  }
}