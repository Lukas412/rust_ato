use crate::core::build::error::BuildError;
use crate::{GeneralCreation, PackProvider, Requirements};

pub trait BuildableWithRequirements<B> {
  fn to_requirement_box(self) -> GeneralCreation;

  fn build(self, pack_provider: &PackProvider, requirements: &mut Requirements) -> Result<B, BuildError>;
}