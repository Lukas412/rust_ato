use crate::core::build::error::BuildError;
use crate::core::main::general::requirements::RequirementBox;

pub trait BuildableWithRequirements<B, R> {
  fn to_requirement_box(self) -> RequirementBox;

  fn build(&self, requirements: &R) -> Result<B, BuildError>;
}