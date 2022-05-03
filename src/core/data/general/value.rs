use crate::core::data::boolean::value::BooleanValue;
use crate::core::data::number::value::NumberValue;
use crate::core::data::path::value::PathValue;
use crate::core::data::string::value::StringValue;
use crate::core::traits::element::Value;

pub enum GeneralValue {
  Boolean(BooleanValue),
  Number(NumberValue),
  Path(PathValue),
  String(StringValue),
}

impl Value<Option<bool>> for GeneralValue {
  fn new(value: bool, namespace: String) -> Self {
    GeneralValue::Boolean(BooleanValue::new(value, namespace))
  }

  fn value(&self) -> Option<&bool> {
    match self {
      
      _ => None
    }
  }

  fn namespace(&self) -> &String {
    todo!()
  }
}