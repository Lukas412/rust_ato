use yaserde::YaDeserialize;

pub trait File {
  fn suffix() -> String;
}