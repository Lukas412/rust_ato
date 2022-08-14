pub(crate) mod value;

use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};
use crate::core::variant::Variant;

#[derive(Debug)]
pub(crate) struct OperationNotFoundError {
  name: String
}

impl OperationNotFoundError {
  pub(crate) fn new_report(name: String) -> Report<Self> {
    report!(Self::new(name))
  }

  pub(crate) fn new(name: String) -> Self {
    Self { name }
  }
}

impl Display for OperationNotFoundError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Operation could not be found: '{}'", self.name)
  }
}

impl Context for OperationNotFoundError {}


#[derive(Debug)]
pub(crate) struct WrongVariantError {
  expected_variant: Variant,
  actual_variant: Variant
}

impl WrongVariantError {
  pub(crate) fn new_report(expected_variant: Variant, actual_variant: Variant) -> Report<Self> {
    report!(Self::new(expected_variant, actual_variant))
  }

  pub(crate) fn new(expected_variant: Variant, actual_variant: Variant) -> Self {
    Self { expected_variant, actual_variant }
  }
}

impl Display for WrongVariantError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Wrong Variant: expected variant '{}' but found '{}'", self.expected_variant, self.actual_variant)
  }
}

impl Context for WrongVariantError {}