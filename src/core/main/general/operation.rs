use crate::core::main::boolean::operation::BooleanOperation;
use crate::core::main::number::operation::NumberOperation;
use crate::core::main::path::operation::PathOperation;
use crate::core::main::string::operation::StringOperation;

pub mod empty;

pub enum CombinedGeneralOperation {
  Action,
  Boolean(BooleanOperation),
  Number(NumberOperation),
  Path(PathOperation),
  String(StringOperation),
}