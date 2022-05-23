use crate::core::build::error::BuildError;
use crate::core::main::general::requirements::RequirementBox;
use crate::Requirements;

pub trait Buildable<B> {
  fn build(&self, requirements: &Requirements) -> Result<B, BuildError>;
}

pub trait BuildableWithRequirements<B> {
  fn to_requirement_box(self) -> RequirementBox;

  fn build(self, requirements: &Requirements) -> Result<B, BuildError>;
}