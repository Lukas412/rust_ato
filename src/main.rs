#[macro_use]
extern crate yaserde_derive;

use yaserde::de::from_str;

use crate::core::element::string::operation::StringValueOperation;
use self::core::element::{Element, Operation};

mod core;

fn main() {
  let test = "<string:value xmlns:string=\"http://www.ato.net/xmlns/element/string\">Hey</string:value>";
  let xml: StringValueOperation = from_str(test).unwrap();

  println!("{}", xml.build().get_value());
}
