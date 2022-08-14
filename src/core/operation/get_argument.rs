use error_stack::ResultExt;
use crate::core::namespace::ParameterName;
use crate::core::value::Value;
use crate::core::variant::Variant;
use crate::{CreationStack, PackProvider};
use crate::errors::build::BuildError;

pub(crate) fn build_get_argument(variant: &Variant, pack_provider: &PackProvider, stack: &mut CreationStack, name: &ParameterName) -> error_stack::Result<Value, BuildError> {
  let operation = stack.get_operation(name)
    .change_context(BuildError::default())?;

  operation.expect_is_variant(variant)
    .change_context(BuildError::default())?;

  operation.build(pack_provider, stack)
}