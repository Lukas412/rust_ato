use std::error::Error;
use std::str::FromStr;

use error_stack::{Context, IntoReport, ResultExt};

use crate::core::namespace::Namespace;
use crate::core::value::{Data, Value};
use crate::core::variant::Variant;
use crate::CreationStack;
use crate::errors::attachments::VariantInformation;
use crate::errors::build::BuildError;
use crate::errors::build::operation::value::ValueParseError;

pub(crate) fn build_value(variant: &Variant, stack: &CreationStack, text: &String) -> error_stack::Result<Value, BuildError> {
  let namespace = stack.get_namespace()
    .change_context(BuildError::default())?;

  let data = get_data(variant, text, &namespace)
    .attach_printable(VariantInformation::new(variant.clone()))
    .change_context(BuildError::default())?;

  Ok(Value::new(data, namespace))
}

fn get_data(variant: &Variant, text: &String, namespace: &Namespace) -> error_stack::Result<Data, ValueParseError> {
  let data = match variant {
    Variant::Action => Data::Action(from_text(text)?),
    Variant::Boolean => Data::Boolean(from_text(text)?),
    Variant::Number => Data::Number(from_text(text)?),
    Variant::Path => Data::Path(from_text(text)?),
    Variant::String => Data::String(from_text(text)?),
    Variant::None => Data::None
  };
  Ok(data)
}

fn from_text<T: FromStr>(text: &String) -> error_stack::Result<T, ValueParseError> {
  T::from_str(text)
    .map_err(|_| ValueParseError::new_report(text.clone()))
}