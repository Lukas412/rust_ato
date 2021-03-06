use crate::{BuildError, GeneralCreation, GeneralCreationStack};
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::main::string::value::StringValue;
use crate::core::traits::operation::Operation;

pub mod empty;

#[derive(Debug)]
pub enum GeneralOperation {
  Empty,
  Value(String),
  Operation(Vec<GeneralCreation>),
}

impl Operation<StringValue> for GeneralOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut GeneralCreationStack) -> Result<StringValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(string) => todo!(),
      Self::Operation(creations) => todo!(),
    }
  }
}