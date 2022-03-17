use std::convert::Infallible;
use std::path::PathBuf;
use std::str::{FromStr, Split};

pub struct Namespace {
  value: String,
}

impl Namespace {
  pub fn new(value: String) -> Namespace {
    Namespace { value }
  }
}