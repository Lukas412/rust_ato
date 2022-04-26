use crate::core::data::element::boolean::element::BooleanElement;
use crate::core::data::element::number::element::NumberElement;
use crate::core::data::element::path::element::PathElement;
use crate::core::data::element::string::element::StringElement;

pub mod boolean;
pub mod number;
pub mod path;
pub mod string;
pub mod parameter;
pub mod creation;

#[derive(Debug)]
pub enum Element {
  Boolean(BooleanElement),
  Number(NumberElement),
  Path(PathElement),
  String(StringElement),
}
