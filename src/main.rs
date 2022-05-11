#[macro_use]
extern crate yaserde_derive;

use std::fs::read_to_string;

use yaserde::de::from_str;
use crate::core::main::general::pack::PackProvider;
use crate::core::traits::build::Buildable;


use self::core::main::general::creation::GeneralCreation;

mod core;

fn main() {
  let pack_provider = PackProvider::from_root("src/bundles".as_ref());
  let string = read_to_string("src/creations/empty-git.creation.xml").unwrap();
  let creation: GeneralCreation = from_str(&string).unwrap();
  let value = creation.build(&pack_provider).unwrap();
  println!("{:?}", value);
}