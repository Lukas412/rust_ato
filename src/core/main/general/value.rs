use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use rust_decimal::Decimal;

use crate::core::main::action::value::Action;
use crate::core::traits::namespace::Namespace;

#[derive(Debug)]
pub struct Value<T> {
  value: T,
  namespace: Namespace,
}

impl<T> Value<T>
  where T: Default
{
  pub fn new(value: T, namespace: Namespace) -> Self {
    Self { value, namespace }
  }

  pub fn default_with_namespace(namespace: Namespace) -> Self {
    let value = T::default();
    Self { value, namespace }
  }

  pub fn value(&self) -> &T {
    &self.value
  }
}

impl<T> Display for Value<T>
  where T: Display
{
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Value({} from {})", self.value, self.namespace)
  }
}