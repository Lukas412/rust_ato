extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use std::path::Path;
use crate::core::creation::Creation;
use crate::core::creation::stack::CreationStack;
use crate::core::pack::provider::PackProvider;
use crate::core::parse::from_file;


use self::core::main::general::creation::Creation;

mod core;

fn main() {
  let pack_provider = PackProvider::from_root("src/bundles");
  build(&pack_provider, "src/creations/test2.creation.xml");
}

fn build<P: AsRef<Path>>(pack_provider: &PackProvider, file: P) {
  let mut requirements = CreationStack::default();
  let creation: Creation = from_file(file).unwrap();
  println!("{:?}", creation);

  let value = creation.build(&pack_provider, &mut requirements);
  match value {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error)
  }
}