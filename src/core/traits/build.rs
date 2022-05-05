pub trait Buildable<Build, Error, Requirements> {
  fn build(&self, requirements: &Requirements) -> Result<Build, Error>;
}