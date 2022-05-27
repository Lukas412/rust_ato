use crate::BuildError;

pub trait GetBuildError {
  fn build_error(&self) -> BuildError;
}