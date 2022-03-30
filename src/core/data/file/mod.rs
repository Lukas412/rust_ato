pub mod pack;

pub trait File {
  fn suffix() -> String;
}