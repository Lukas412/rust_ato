use crate::{BuildError, GeneralCreation, CreationStack};
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::general::pack::provider::PackProvider;
use crate::core::main::string::value::StringValue;
use crate::core::traits::namespace::Namespace;

pub mod empty;

#[derive(Debug)]
pub enum Operation {
  Empty,
  Value(String),
  GetArgument {
    name: String,
    namespace: Namespace,
  },
}

impl Operation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut CreationStack) -> Result<StringValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(string) => todo!(),
      Self::GetArgument { name, namespace } => todo!(),
    }
  }
}