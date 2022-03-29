#[macro_use]
extern crate yaserde_derive;

use yaserde::de::from_str;
use crate::core::element::boolean::operation::BooleanValueOperation;

mod core;
mod concepts;

fn main() {
  let test: BooleanValueOperation = from_str("<boolean:value xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\">Hello</boolean:value>").unwrap();
  println!("{:?}", test);
}
