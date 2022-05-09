use crate::{Container, Value};
use crate::core::build::error::BuildError;
use crate::core::main::string::value::StringValue;
use crate::core::main::string::operation::get_argument::StringGetArgumentOperation;
use crate::core::main::string::operation::value::StringValueOperation;
use crate::core::traits::build::Buildable;
use crate::core::traits::container::Provide;

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

impl<C: Container + Provide<StringValue, BuildError>> Buildable<StringValue, BuildError, C> for StringOperation {
  fn build(&self, requirements: &C) -> Result<StringValue, BuildError> {
    match self {
      Self::Empty => Ok(StringValue::new("".to_owned(), requirements.namespace().to_owned())),
      Self::Value(operation) => operation.build(requirements),
      Self::GetArgument(operation) => operation.build(requirements),
    }
  }
}