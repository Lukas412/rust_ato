use std::str::FromStr;

use crate::{BuildError, CreationStack};
use crate::core::main::general::value::{Data, Value};
use crate::core::main::general::variant::Variant;
use crate::core::traits::namespace::{Namespace};

pub fn build_value(variant: &Variant, stack: &CreationStack, text: &String) -> Result<Value, BuildError> {
  let data = get_data(variant, text, stack.get_namespace())?;
  let namespace = stack.get_owned_namespace();
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
    };
  Ok(data)
}

fn from_text<T: FromStr>(text: &String, namespace: &Namespace) -> Result<T, BuildError> {
  match T::from_str(text) {
    Ok(result) => Ok(result),
    Err(_) => BuildError::new_can_not_convert_text_to_value_error(&text, namespace.to_owned()),
  }
}