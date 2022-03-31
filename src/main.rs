#[macro_use]
extern crate yaserde_derive;

use std::fs::read_to_string;
use yaserde::de::from_str;

mod core;

fn main() {
  let string = read_to_string("src/bundles/git/commit/message.pack.xml").unwrap();
  println!("{}", string);

  let test = from_str(&string).unwrap();
  println!("{:?}", test);
}
