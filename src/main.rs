extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate walkdir;
extern crate rust_decimal;

use core::builder::Builder;

use crate::core::creation::Creation;
use crate::core::creation::stack::CreationStack;
use crate::core::pack::provider::PackProvider;
use crate::core::parse::from_file;

mod core;
mod common;

fn main() {
  let builder = Builder::new("src/bundles");
  let value = builder.build_creation("src/creations/test2.creation.xml");
  match value {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error)
  }
}