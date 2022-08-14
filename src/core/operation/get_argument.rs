use std::borrow::Borrow;
use std::rc::Rc;
use error_stack::{ensure, ResultExt};
use crate::core::namespace::ParameterName;
use crate::core::value::Value;
use crate::core::variant::Variant;
use crate::{CreationStack, PackProvider};
use crate::core::operation::Operation;
use crate::errors::build::BuildError;
use crate::errors::build::operation::WrongVariantError;

pub(crate) fn build_get_argument(variant: &Variant, pack_provider: &PackProvider, stack: &mut CreationStack, name: &ParameterName) -> error_stack::Result<Value, BuildError> {
  let operation = stack.get_operation(name)
    .change_context(BuildError::default())?;

  ensure_operation_variant(operation.borrow(), variant)
    .change_context(BuildError::default())?;

  operation.build(pack_provider, stack)
}

fn ensure_operation_variant(operation: &Operation, variant: &Variant) -> error_stack::Result<(), WrongVariantError> {
  ensure!(
    operation.is_variant(variant),
    WrongVariantError::new(variant.clone(), operation.variant.clone())
  );
  Ok(())
}