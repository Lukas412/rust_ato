use std::collections::HashMap;
use crate::core::main::boolean::operation::BooleanOperation;
use crate::core::main::general::value::CombinedGeneralValue;
use crate::core::main::number::operation::NumberOperation;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::string::operation::StringOperation;
use crate::core::traits::operation::Operation;

pub mod empty;

#[derive(Debug, Default)]
pub struct GeneralOperationProvider {
  boolean_operations: HashMap<String, BooleanOperation>,
  number_operations: HashMap<String, NumberOperation>,
  path_operations: HashMap<String, PathOperation>,
  string_operations: HashMap<String, StringOperation>,
}

#[derive(Debug)]
pub enum CombinedGeneralOperation {
  None,
  Action,
  Boolean(BooleanOperation),
  Number(NumberOperation),
  Path(PathOperation),
  String(StringOperation),
}

impl Default for CombinedGeneralOperation {
  fn default() -> Self {
    Self::None
  }
}

impl Operation for CombinedGeneralOperation {
  type Value = CombinedGeneralValue;
}