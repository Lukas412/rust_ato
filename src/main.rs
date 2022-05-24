extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::path::Path;
use crate::core::main::general::pack::GeneralPackProvider;
use crate::core::main::general::requirements::Requirements;
use crate::core::parse::from_file;
use crate::core::traits::build::BuildableWithRequirements;


use self::core::main::general::creation::GeneralCreation;

mod core;

static PACK_PROVIDER: GeneralPackProvider = GeneralPackProvider::from_root("src/bundles");

fn main() {
  build("src/creations/test2.creation.xml");
}

fn build<P: AsRef<Path>>(file: P) {
  let requirements = Requirements::new(&PACK_PROVIDER);
  let creation: GeneralCreation = from_file(file).unwrap();
  println!("{:?}", creation);

  let value = creation.build(&requirements).unwrap();
  println!("{:?}", value);
}