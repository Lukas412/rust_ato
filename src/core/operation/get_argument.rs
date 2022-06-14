use crate::core::error::BuildError;
use crate::core::namespace::ParameterName;
use crate::core::value::Value;
use crate::core::variant::Variant;
use crate::CreationStack;

pub fn build_get_argument(variant: &Variant, stack: &CreationStack, name: &ParameterName) -> Result<Value, BuildError> {
  let creation = stack.last()?;
}