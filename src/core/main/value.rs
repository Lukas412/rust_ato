use std::fmt::{Display, Formatter, write};
use std::path::PathBuf;
use rust_decimal::Decimal;
use crate::core::main::action::value::Action;

use crate::core::main::namespace::Namespace;

#[derive(Debug)]
pub struct Value {
  data: Data,
  namespace: Namespace,
}

impl Value {
  pub fn new(value: Data, namespace: Namespace) -> Self {
    Self { data: value, namespace }
  }

  pub fn default_with_namespace(namespace: Namespace) -> Self {
    let data = Data::default();
    Self { data, namespace }
  }

  pub fn value(&self) -> &Data {
    &self.data
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Value({} from {})", self.data, self.namespace)
  }
}

#[derive(Debug)]
pub enum Data {
  None,
  Action(Action),
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Default for Data {
  fn default() -> Self {
    Self::None
  }
}

impl Display for Data {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match &self {
      Data::None => write!(f, "None"),
      Data::Action(action) => write!(f, "Action({})", action),
      Data::Boolean(boolean) => write!(f, "Boolean({})", boolean),
      Data::Number(number) => write!(f, "Number({})", number),
      Data::Path(path) => write!(f, "Path({})", path.display()),
      Data::String(string) => write!(f, "String({})", string),
    }
  }
}

#[derive(Debug, Clone)]
pub enum Action {
  None,
  Content,
  Directory,
  Expression,
  File,
  Location,
  Output,
}

impl Default for Action {
  fn default() -> Self {
    Self::None
  }
}

impl Display for Action {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Action")
  }
}