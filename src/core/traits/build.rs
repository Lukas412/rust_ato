use crate::core::build::error::BuildError;
use crate::core::main::general::requirements::RequirementBox;
use crate::{PackProvider, Requirements};

pub trait BuildableWithRequirements<B> {
  fn to_requirement_box(self) -> RequirementBox;

  fn build(self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<B, BuildError>;
}