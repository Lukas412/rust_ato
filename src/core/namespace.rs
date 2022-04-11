#[derive(Debug)]
pub struct Namespace(String);

impl Namespace {
  pub fn empty() -> Namespace {
    Namespace("".to_string())
  }
}