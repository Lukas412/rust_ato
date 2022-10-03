use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Variant {
  None,
  Action,
  Boolean,
  Number,
  Path,
  String,
}

impl Default for Variant {
  fn default() -> Self {
    Self::None
  }
}

impl Display for Variant {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Variant::None => write!(f, "None"),
      Variant::Action => write!(f, "Action"),
      Variant::Boolean => write!(f, "Boolean"),
      Variant::Number => write!(f, "Number"),
      Variant::Path => write!(f, "Path"),
      Variant::String => write!(f, "String")
    }
  }
}
