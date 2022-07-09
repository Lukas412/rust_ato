extern crate yaserde;
#[macro_use]
extern crate yaserde_derive;
extern crate walkdir;
extern crate rust_decimal;

use std::rc::Rc;
use core::builder::Builder;

use crate::core::creation::Creation;
use crate::core::creation::stack::CreationStack;
use crate::core::pack::provider::PackProvider;

mod core;
mod common;

fn main() {
  let builder = Builder::from_root("src/bundles").unwrap();
  let value = builder.build_creation("src/creations/test2.creation.xml");
  match value {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error)
  }
}