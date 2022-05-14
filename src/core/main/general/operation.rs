use crate::core::main::boolean::operation::BooleanOperation;
use crate::core::main::general::value::CombinedGeneralValue;
use crate::core::main::number::operation::NumberOperation;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::string::operation::StringOperation;
use crate::core::traits::operation::Operation;

pub mod empty;

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