#[macro_use]
extern crate yaserde_derive;

use std::fs::read_to_string;
use yaserde::de::from_str;
use crate::core::data::element::boolean::operation::BooleanValueOperation;

mod core;

fn main() {
  // let string = read_to_string("src/bundles/git/commit/message.pack.xml").unwrap();
  let string = "<boolean:value xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\">Hi</boolean:value>";
  println!("{}", string);

  let test: BooleanValueOperation = from_str(string).unwrap();
  println!("{:?}", test);
}
