use std::io::Read;
use std::rc::Rc;

use serde::Deserialize;

use crate::core::operation::Operation;
use crate::core::variant::Variant;
use crate::Creation;

#[derive(Debug)]
pub(crate) struct CreationValue {
  name: String,
  operation: Rc<Operation>,
}

impl CreationValue {
  pub(crate) fn to_name_and_operation(self) -> (String, Rc<Operation>) {
    (self.name, self.operation)
  }
}

#[derive(Debug, Deserialize)]
pub(crate) struct InnerCreationValue {
  name: String,
  value: Option<String>,
  creation: Option<Creation>,
}

impl InnerCreationValue {
  fn to_name_and_rc_operation(self, variant: Variant) -> (String, Rc<Operation>) {
    let (name, operation) =
      match self {
        InnerCreationValue { name, value: Some(value), .. } =>
          (name, Operation::new_value(value, variant)),
        InnerCreationValue { name, creation: Some(creation), .. } => {
          let creation = Rc::new(creation);
          (name, Operation::new_creation(creation, variant))
        }
        InnerCreationValue { name, .. } =>
          (name, Operation::new_empty(variant))
      };
    let operation = Rc::new(operation);
    (name, operation)
  }
}
