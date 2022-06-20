use std::str::FromStr;
use crate::core::error::BuildError;
use crate::core::namespace::Namespace;
use crate::core::value::{Data, Value};
use crate::core::variant::Variant;

use crate::CreationStack;

pub fn build_value(variant: &Variant, stack: &CreationStack, text: &String) -> Result<Value, BuildError> {
  let namespace = stack.get_owned_namespace();
  let data = get_data(variant, text, &namespace)?;
  Ok(Value::new(data, namespace))
}

fn get_data(variant: &Variant, text: &String, namespace: &Namespace) -> Result<Data, BuildError> {
  let data =
    match variant {
      Variant::Action => Data::Action(from_text(text, namespace)?),
      Variant::Boolean => Data::Boolean(from_text(text, namespace)?),
      Variant::Number => Data::Number(from_text(text, namespace)?),
      Variant::Path => Data::Path(from_text(text, namespace)?),
      Variant::String => Data::String(from_text(text, namespace)?),
      Variant::None => Data::None
    };
  Ok(data)
}

fn from_text<T: FromStr>(text: &String, namespace: &Namespace) -> Result<T, BuildError> {
  match T::from_str(text) {
    Ok(result) => Ok(result),
    Err(_) => Err(BuildError::new_can_not_convert_text_to_value_error(&text, namespace.to_owned())),
  }
}