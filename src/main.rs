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
mod yamoreserde;

fn main() {
  let builder = Builder::from_root("src/bundles").unwrap();
  let mut build = builder.create_build("src/creations/test2.creation.xml").unwrap();
  let value = build.build();
  match value {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error)
  }
}