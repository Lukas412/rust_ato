#[macro_use]
extern crate yaserde_derive;

use std::fs::read_to_string;

use crate::core::main::general::pack::PackProvider;
use crate::core::parse::from_file;
use crate::core::traits::build::Buildable;


use self::core::main::general::creation::GeneralCreation;

mod core;

fn main() {
  let pack_provider = PackProvider::from_root("src/bundles".as_ref());
  let creation: GeneralCreation = from_file("src/creations/empty-git.creation.xml").unwrap();
  let value = creation.build(&pack_provider).unwrap();
  println!("{:?}", value);
}