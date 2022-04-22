#[derive(Debug, Default, YaDeserialize)]
pub struct ElementArguments {
  #[yaserde(rename = "argument")]
  arguments: Vec<ElementArgument>,
}

#[derive(Debug)]
pub enum ElementArgument {
  Boolean(),
  Number(),
  Path(),
  String(),
}