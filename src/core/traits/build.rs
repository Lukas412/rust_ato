pub trait Buildable<B, E, R> {
  fn build(&self, requirements: &R) -> Result<B, E>;
}