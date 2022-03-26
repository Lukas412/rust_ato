#[macro_use]
extern crate yaserde_derive;

use yaserde::de::from_str;

use self::core::element::boolean::parameter::BooleanParameter;

mod core;

fn main() {
  let test = "<boolean:parameter xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\" name=\"yes\"/>";
  let xml: Pack<> = from_str(test).unwrap();
  println!("{:?}", xml)
}
