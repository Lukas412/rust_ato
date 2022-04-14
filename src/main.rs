#[macro_use]
extern crate yaserde_derive;

use std::fs::read_to_string;
use yaserde::de::from_str;
use crate::core::data::element::string::pack::StringPack;

mod core;

fn main() {
  let string = read_to_string("src/bundles/angular/cli/build/production.string.xml").unwrap();
  // let string = "<boolean:value xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\">Hi</boolean:value>";
  println!("{}", string);

  let test: StringPack = from_str(&string).unwrap();
  println!("{:?}", test);
}
