use crate::core::build::error::BuildError;

pub trait Buildable<B, R> {
  fn build(&self, requirements: &R) -> Result<B, BuildError>;
}