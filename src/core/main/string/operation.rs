use crate::core::build::error::BuildError;
use crate::core::main::general::operation::empty::build_empty;
use crate::core::main::string::operation::get_argument::StringGetArgumentOperation;
use crate::core::main::string::operation::value::StringValueOperation;
use crate::core::main::string::value::StringValue;
use crate::core::traits::operation::Operation;
use crate::GeneralCreationStack;
use crate::core::main::general::pack::provider::PackProvider;

pub mod value;
pub mod get_argument;

#[derive(Debug, YaDeserialize)]
#[yaserde(prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
pub enum StringOperation {
  #[yaserde(rename = "empty", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
  Empty,
  #[yaserde(rename = "value", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
  Value(StringValueOperation),
  #[yaserde(rename = "get_argument", prefix = "string", namespace = "string: http://www.ato.net/xmlns/string")]
  GetArgument(StringGetArgumentOperation),
}

impl Default for StringOperation {
  fn default() -> Self {
    Self::Empty
  }
}

impl Operation<StringValue> for StringOperation {
  fn build(&self, pack_provider: &PackProvider, requirements: &mut GeneralCreationStack) -> Result<StringValue, BuildError> {
    match self {
      Self::Empty => build_empty(requirements),
      Self::Value(operation) => operation.build(pack_provider, requirements),
      Self::GetArgument(operation) => operation.build(pack_provider, requirements),
    }
  }
}