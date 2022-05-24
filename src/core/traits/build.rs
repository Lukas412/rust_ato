use crate::core::build::error::BuildError;
use crate::core::main::general::requirements::RequirementBox;
use crate::{PackProvider, Requirements};

pub trait Buildable<B> {
  fn build(&self, pack_provider: &PackProvider, requirements: &Requirements) -> Result<B, BuildError>;
}

pub trait BuildableWithRequirements<B> {
  fn to_requirement_box(self) -> RequirementBox;

  fn build(self, pack_provider: &PackProvider, requirements: &Requirements) -> Result<B, BuildError>;
}