pub trait Parameter {
  fn name(&self) -> String;
  fn namespace(&self) -> String;
}