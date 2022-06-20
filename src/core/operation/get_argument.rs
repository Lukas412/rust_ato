use crate::core::error::BuildError;
use crate::core::namespace::ParameterName;
use crate::core::value::Value;
use crate::core::variant::Variant;
use crate::{CreationStack, PackProvider};

pub fn build_get_argument(variant: &Variant, pack_provider: &PackProvider, stack: &mut CreationStack, name: &ParameterName) -> Result<Value, BuildError> {
  let operation = stack.get_operation(name)?;
  if operation.is_variant(variant) {
    operation.build(pack_provider, stack)
  } else {
    todo!()
  }
}