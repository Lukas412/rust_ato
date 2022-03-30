#[macro_use]
extern crate yaserde_derive;

use yaserde::de::from_str;
use crate::core::concepts::build::BuildableWithRequirements;
use crate::core::data::element::boolean::operation::BooleanValueOperation;
use crate::core::data::requirement::Requirements;

mod core;

fn main() {
  let test: BooleanValueOperation = from_str("<boolean:value xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\">false</boolean:value>").unwrap();
  println!("{:?}", test.build_with_requirements(Requirements::empty()));
}
