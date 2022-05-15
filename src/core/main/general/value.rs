use std::collections::HashMap;
use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::core::main::action::value::Action;
use crate::core::traits::namespace::Namespace;
use crate::core::traits::value::Value;

#[derive(Debug, Clone)]
pub enum CombinedGeneralValue {
  None,
  Action(Action),
  Boolean(bool),
  Number(Decimal),
  Path(PathBuf),
  String(String),
}

impl Default for CombinedGeneralValue {
  fn default() -> Self {
    Self::None
  }
}
