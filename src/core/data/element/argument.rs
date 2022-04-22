#[derive(Debug, Default, YaDeserialize)]
pub struct Arguments {
  #[yaserde(rename = "argument")]
  arguments: Vec<Argument>,
}

#[derive(Debug)]
pub enum Argument {
  Boolean(),
  Number(),
  Path(),
  String(),
}