extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::path::Path;
use crate::core::main::general::pack::GeneralPackProvider;
use crate::core::main::general::requirements::Requirements;
use crate::core::parse::from_file;
use crate::core::traits::build::Buildable;


use self::core::main::general::creation::InnerGeneralCreation;

mod core;

fn main() {
  build("src/creations/test2.creation.xml");
}

fn build<P: AsRef<Path>>(file: P) {
  let pack_provider = GeneralPackProvider::from_root("src/bundles");
  let requirements = Requirements::new(&pack_provider);
  let creation: InnerGeneralCreation = from_file(file).unwrap();
  println!("{:?}", creation);

  let value = creation.build(&requirements).unwrap();
  println!("{:?}", value);
}