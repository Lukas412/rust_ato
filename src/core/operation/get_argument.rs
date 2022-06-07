use crate::CreationStack;
use crate::core::error::BuildError;
use crate::core::namespace::Namespace;
use crate::core::value::Value;
use crate::core::variant::Variant;

pub fn build_get_argument(variant: &Variant, stack: &CreationStack, name: &String, namespace: &Namespace) -> Result<Value, BuildError> {
  let creation = stack.last()?;
  creation.operation()
}