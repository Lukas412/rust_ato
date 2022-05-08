#[macro_use]
extern crate yaserde_derive;

use std::fs::read_to_string;

use yaserde::de::from_str;
use crate::core::main::element::container::ElementContainer;
use crate::core::main::element::value::{CombinedElementValue, ElementValue};

use crate::core::main::string::pack::StringPack;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Container;
use crate::core::traits::pack::Pack;
use crate::core::traits::value::Value;

mod core;

fn main() {
  let string = read_to_string("src/bundles/angular/cli/init/name.string.xml").unwrap();
  // let string = "<boolean:value xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\">Hi</boolean:value>";
  println!("{}", string);

  let test: StringPack = from_str(&string).unwrap();
  let elements = [("value".to_owned(), ElementValue::new(CombinedElementValue::String("website-name".to_owned()), "__creation__".to_owned())); 1];
  println!("{:#?}", test.build_with_requirements(elements));
}
