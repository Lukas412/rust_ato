use yaserde::YaDeserialize;

pub trait File: YaDeserialize {
  fn suffix() -> String;
}