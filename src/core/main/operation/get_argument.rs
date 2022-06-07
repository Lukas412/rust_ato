use crate::{BuildError, CreationStack, Creation};
use crate::core::main::general::value::{Data, Value};
use crate::core::main::general::variant::Variant;
use crate::core::main::namespace::Namespace;

pub fn build_get_argument(variant: &Variant, stack: &CreationStack, name: &String, namespace: &Namespace) -> Result<Value, BuildError> {
  let creation = stack.last()?;
  todo!()
}