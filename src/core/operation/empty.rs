use crate::core::value::{Data, Value};
use crate::core::variant::Variant;
use crate::CreationStack;
use crate::errors::build::BuildError;

pub(crate) fn build_empty(variant: &Variant, stack: &CreationStack) -> Result<Value, BuildError> {
  let data =
    match variant {
      Variant::Action => Data::Action(create_default()),
      Variant::Boolean => Data::Boolean(create_default()),
      Variant::Number => Data::Number(create_default()),
      Variant::Path => Data::Path(create_default()),
      Variant::String => Data::String(create_default()),
      Variant::None => Data::None
    };
  let namespace = stack.get_owned_namespace();
  Ok(Value::new(data, namespace))
}

fn create_default<T: Default>() -> T {
  T::default()
}