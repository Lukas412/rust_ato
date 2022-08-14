#![allow(unused, dead_code)]

extern crate rust_decimal;
extern crate walkdir;
extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;

use error_stack::ResultExt;
use crate::core::build::Build;
use crate::core::builder::Builder;

use crate::core::creation::Creation;
use crate::core::creation::stack::CreationStack;
use crate::core::pack::provider::PackProvider;

mod core;
mod helpers;
mod errors;

fn main() {
  let builder = Builder::from_root("src/bundles").unwrap();

  let mut build = match builder.create_build("src/creations/test2.creation.xml") {
    Ok(build) => build,
    Err(error) => panic!("\n{:?}", error)
  };

  let value = build.build();
  match value {
    Ok(value) => println!("{}", value),
    Err(error) => panic!("\n{:?}", error)
  }
}