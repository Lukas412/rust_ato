use yaserde::YaDeserialize;

pub mod pack;

pub trait File: YaDeserialize {
  fn suffix() -> String;
}