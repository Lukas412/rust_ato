use crate::core::main::string::operation::StringOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::build::Buildable;
use crate::core::traits::operation::GetOperation;
use crate::{BuildError, GeneralCreation, PackProvider, Requirements};
use crate::core::main::general::operation::empty::build_empty;

pub mod empty;

#[derive(Debug)]
pub enum GeneralOperation {
  Empty,
  Value(String),
  Operation(Vec<GeneralCreation>),
}

impl Buildable<StringValue> for GeneralOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<StringValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(operation) => todo!(),
      Self::Operation(operation) => todo!(),
    }
  }
}