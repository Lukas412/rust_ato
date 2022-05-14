#[macro_use]
extern crate yaserde_derive;

use crate::core::main::general::pack::GeneralPackProvider;
use crate::core::parse::from_file;
use crate::core::traits::build::Buildable;


use self::core::main::general::creation::GeneralCreation;

mod core;

fn main() {
  let pack_provider = GeneralPackProvider::from_root("src/bundles".as_ref());
  let creation: GeneralCreation = from_file("src/creations/empty-git.creation.xml").unwrap();
  let value = creation.build(&pack_provider).unwrap();
  println!("{:?}", value);
}